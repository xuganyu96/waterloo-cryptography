"""Digital Signature Algorithm based on the ElGamal cryptosystem
"""
import os
import random
import sympy

P2048 = int("".join((
    "FFFFFFFF", "FFFFFFFF", "ADF85458", "A2BB4A9A", "AFDC5620", "273D3CF1", "D8B9C583",
    "CE2D3695", "A9E13641", "146433FB", "CC939DCE", "249B3EF9", "7D2FE363", "630C75D8",
    "F681B202", "AEC4617A", "D3DF1ED5", "D5FD6561", "2433F51F", "5F066ED0", "85636555",
    "3DED1AF3", "B557135E", "7F57C935", "984F0C70", "E0E68B77", "E2A689DA", "F3EFE872",
    "1DF158A1", "36ADE735", "30ACCA4F", "483A797A", "BC0AB182", "B324FB61", "D108A94B",
    "B2C8E3FB", "B96ADAB7", "60D7F468", "1D4F42A3", "DE394DF4", "AE56EDE7", "6372BB19",
    "0B07A7C8", "EE0A6D70", "9E02FCE1", "CDF7E2EC", "C03404CD", "28342F61", "9172FE9C",
    "E98583FF", "8E4F1232", "EEF28183", "C3FE3B1B", "4C6FAD73", "3BB5FCBC", "2EC22005",
    "C58EF183", "7D1683B2", "C6F34A26", "C1B2EFFA", "886B4238", "61285C97", "FFFFFFFF",
    "FFFFFFFF",
)), base=16);

class Params:
    """DSA is parameterized by three numbers p, q, g. g is an element in the
    multiplicative group Z_p with prime order q
    """
    def __init__(self, p: int, q: int, g: int):
        """Initialize a set of parameters with minimal checks"""
        if not sympy.ntheory.isprime(p):
            raise ValueError("p is not prime")
        if not sympy.ntheory.isprime(q):
            raise ValueError("q is not prime")
        if (p - 1) % q != 0:
            raise ValueError(f"Invalid prime pairs {p}, {q}")
        if pow(g, q, p) != 1:
            raise ValueError(f"Order of {g} does not divide q")
        self.p = p
        self.q = q
        self.g = g
    
    @staticmethod
    def gen2048():
        """Generate using the recommended 2048 bits parameters"""
        p = P2048
        q = (p - 1) // 2
        g = 2
        return Params(p, q, g)

class PublicKey:
    """A public key is an element of the multiplicative gropu Z_p"""
    def __init__(self, val: int, params: Params):
        self.val = val
        self.params = params
    
    def __repr__(self):
        return f"<PublicKey 0x{self.val:X}>"

class SecretKey:
    """A private key is an exponent of g, so it is an element of the integer
    ring Z_q since g^q = 1 mod p
    """
    def __init__(self, val: int, params: Params):
        self.val = val
        self.params = params
    
    def __repr__(self):
        return f"<SecretKey 0x{self.val:X}>"

class KeyPair:
    def __init__(self, pk_val: int, sk_val: int, params: Params):
        self.pk = PublicKey(pk_val, params)
        self.sk = SecretKey(sk_val, params)

    @staticmethod
    def keygen(params: Params):
        sk_val = random.randint(1, params.q - 1)
        pk_val = pow(params.g, sk_val, params.p)
        return KeyPair(pk_val, sk_val, params)

class Signature:
    """A signature contains two integer (r, s)"""
    def __init__(self, r_val: int, s_val: int):
        self.r = r_val
        self.s = s_val

def sign_hashed(keypair: KeyPair, m: int, nonce: int | None = None):
    """Sign the hashed message, where the hashed message is in the integer ring
    Z_q. If a nonce is provided then it will be used, otherwise a random nonce
    will be used
    """
    params = keypair.pk.params
    if m < 0 or m >= params.q:
        raise ValueError("Invalid message hash {m}")
    if nonce is None:
        nonce = random.randint(1, params.q - 1)
    nonce_inv = pow(nonce, -1, params.q)
    r = pow(pow(params.g, nonce, params.p), 1, params.q)
    s = pow((m + keypair.sk.val * r) * nonce_inv, 1, params.q)
    return Signature(r, s)

def verify_hashed(
    pk: PublicKey,
    m: int, 
    signature: Signature, 
    bounded: bool = True
):
    """(g^mh^r)^(s_inv) mod p mod q"""
    # Check bound, if bounded is set to True
    if bounded and (signature.r <= 0 
        or signature.r >= pk.params.q 
        or signature.s <= 0 
        or signature.s >= pk.params.q
    ):
        return False
    
    # Check congruence; note that the first reduction by p is necessary because
    # the bound condition can sometimes 
    lhs = pow(signature.r, 1, pk.params.p)
    lhs = pow(lhs, 1, pk.params.q)
    v1 = pow(pk.params.g, m, pk.params.p)
    v2 = pow(pk.val, signature.r, pk.params.p)
    s_inv = pow(signature.s, -1, pk.params.q)
    rhs = pow(pow(v1 * v2, s_inv, pk.params.p), 1, pk.params.q)
    return lhs == rhs

def forge_unbounded(
    m_hashed: int, 
    pk: PublicKey, 
    params: Params, 
    s: int | None = None
):
    """Forge a signature that will pass the no-bound-check verification.
     
    The forged signature randomly samples the second part of the forgery, but
    if a value for (s) is provided, then the provided value will be used

    >> p, q, g = 48731, 443, 5260
    >> sk_val = 242
    >> m_hashed = 343
    >> nonce = 427
    >> params = Params(p, q, g)
    >> keypair = KeyPair(pow(g, sk_val, p), sk_val, params)
    >> sigma = sign_hashed(keypair, m_hashed, nonce)
    >> assert verify_hashed(keypair.pk, m_hashed, sigma, True), "Verification is wrong"
    >> forged = forge_unbounded(m_hashed, keypair.pk, keypair.pk.params, None)
    >> print(verify_hashed(keypair.pk, m_hashed, forged, False))
    """
    if s is None:
        s = random.randint(1, params.q - 1)
    s_inv = pow(s, -1, params.q)
    r_prime = random.randint(1, params.q - 1)
    r_pprime = pow(
        pow(params.g, m_hashed, params.p) * pow(pk.val, r_prime, params.p),
        s_inv,
        params.p
    )
    sols = sympy.ntheory.modular.crt([params.q, params.p], [r_prime, r_pprime])
    r = sols[0]
    
    return Signature(r, s)