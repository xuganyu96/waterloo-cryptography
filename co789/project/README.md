- [Fujisaki-Okamoto transformation](./fujisaki-okamoto.pdf)
- [HPKE](https://datatracker.ietf.org/doc/rfc9180/)
- [A modular approach]?
- [OAEP, and RSA OAEP]

# Introduction
Indistinguishability under (adaptive) chosen-ciphertext attack (IND-CCA2) is widely recognized as the desirable security notion for public-key cryptography. However, directly achieving IND-CCA2 security is difficult. Previous attempts at deploying public-key cryptography in production, such as the usage of RSA PKCS1 v1.5 in early versions of SSl/TLS, were found to be vulnerable to adaptive chosen-ciphertext attacks.

Instead of directly constructing IND-CCA2 secure cryptosystem from NP-hard problems, recent works approached this problem by proposing generic transformation that take cryptographic primitives of lesser strengths (e.g. OW-CPA, IND-CPA) and produce encryption schemes that lose only a negligible amount of security. One such transformation was proposed by Fujisaki and Okamoto in 1999 and later improved by Hofheinz, Hovelmann, and Kiltz in 2017. With simple construction and robust security reduction, the FO transformation is adopted by submissions to NIST's post-quantum cryptography competition (notably by Kyber, which has been standardized in FIPS 203 in 2024).

In this paper, we will review the constructions of the Fujisaki-Okamoto transformation and its variations. We will also review their security results, including the techniques used in the security reduction. Finally, we will discuss open problems and propose some optimization.

# Related works
Another notable example of generic CCA-secure transformation is the Optimal Asymmetric Encryption Padding (OAEP) proposed by Bellare and Rogaway. This scheme made improvements on the simple padding scheme for RSA standardized by PKCS1 v1.5, which was not only easy to implement incorrectly (the simple Bleichenbacher attack), but also later shown to be vulnerable to adaptive chosen-ciphertext attack (the more sophisticated Bleichenbacher attack).

While OAEP was proposed as a generic transformation, it was later shown to be only resilient against CCA1 adversary but not CCA2 adversary.

# From Fujisaki-Okamoto transformation to IND-CCA KEM

## The FO transformation
```python
# PKE is a OW-CPA secure public-key encryption scheme
# G, H are hash functions
# SYM is an IND-CPA secure symmetric-key encryption scheme

def keygen():
    pk, sk = PKE.keygen()
    return pk, sk

def encrypt(pk, m):
    sigma = PKE.random_message()
    a = G(sigma)
    c = SYM(key=a).encrypt(m)
    h = H(sigma, c)
    e = PKE.encrypt(pk, sigma, coin=h)
    return (e, c)

def decrypt(sk, pk, ciphertext):
    e, c = ciphertext
    sigma_hat = PKE.decrypt(sk, e)
    h_hat = H(sigma_hat, c)
    e_hat = PKE.encrypt(pk, sigma_hat, h_hat)
    if e_hat != e:
        raise Error("Invalid ciphertext")
    a_hat = G(sigma_hat)
    return SYM(key=a_hat).decrypt(c)
```

## An outline of the FO transform security proof
Game 0 is the standard IND-CCA2 security game. $\mathcal{A}$'s actions include:
- receives the public key
- has access to a decryption oracle $\mathcal{O}^D_0$ who runs the decryption routine using the true secret key
- generates challenge plaintexts $(m_0^\ast, m_1^\ast)$
- receives challenge ciphertext $c^\ast$
- outputs the guess $b \in \{0, 1\}$

Game 1 is identical to game 0, but the decryption oracle is changed. Instead of using the secret key to decrypt the query, the decryption oracle checks the tape of the hash oracle $\mathcal{O}^H$ for matching queries. $\mathcal{O}^D_1$ behaves differently from $\mathcal{O}^D_0$ when $\mathcal{A}$ produces decryption query $(\tilde{e}, \tilde{c})$ that passes the "re-encryption" routine without querying $H$.

$$
P[S_0] - P[S_1] \leq q_D \cdot 2^{-\gamma}
$$

where $q_D$ is the number of decryption queries, and $\gamma$ is the spread of the PKE ciphertext space.

Game 2 is identical to game 1, except when encrypting the challenge ciphertext, $a^\ast$ and $h^\ast$ are both randomly sampled instead of queried from $G$ and $H$ respectively (note that $e^\ast$ is still an encryption of a randomly sampled $\sigma^\ast$). From $\mathcal{A}$'s perspective, the two games are different if $\mathcal{A}$ ever makes a query to $G$ or $H$ that involves $\sigma^\ast$.

$$
P[S_1] - P[S_2] \leq P[\text{query}]
$$

where $q_H$ is the number of hash queries to either $H$ or $G$, and $P[\text{query}]$ is the probability that $\mathcal{A}$ makes a query that involves $\sigma^\ast$.

Since game 2 doesn't involve the symmetric secret key, it can be played by an IND-CPA adversary against the underlying symmetric cipher:

$$
P[S_2] = P[\mathcal{A}^\text{sym}_\text{IND-CPA} \text{ wins}]
$$

Since game 2 encrypts $\sigma^\ast$ with a truly random coin $h^\ast$, it can be simulated by a OW-CPA adversary against the underlying PKE:

$$
P[\mathcal{A}^\text{asym}_\text{OW-CPA}] = P[\text{query}] \cdot \frac{1}{q_H}
$$

Putting everything together:

$$
\epsilon_\text{CCA} \leq q_D2^{-\gamma} + \epsilon^\text{sym}_\text{IND-CPA} + q_H\epsilon^\text{asym}_\text{OW-CPA}
$$

There are a few drawbacks from this transformation:
- $\epsilon^\text{asym}_\text{OW-CPA}$ has a linear coefficient $q_H$, so the hybrid scheme's security is not tight
- IND-CCA for KEM is different from IND-CCA for PKE
- PKE cannot have decryption error

## A tighter security reduction
First we introduce two new oracles: the plaintext checking oracle $\text{PCO}(m, c)$ and the ciphertext validity oracle $\text{CVO}(c)$. With these two oracles we can define a new security definition called OW/IND-PCVA, where the objective of the adversary is still OW/IND, but now has access to PCO and CVO.

Now a modified PKE scheme:

```python
class HybridPKE:
    def keygen():
        return PKE.keygen()

    def encrypt(pk, m):
        return PKE.encrypt(pk, m, G(m))

    def decrypt(sk, pk, c):
        m = PKE.decrypt(sk, c)
        if PKE.encrypt(pk, m, G(m)) != c:
            raise InvalidCiphertextError()
        return m

class PlaintextChecker:
    """PCO in game 0"""
    def check(pk, sk, m, c):
        m_hat = PKE.decrypt(sk, pk, c)
        assert m_hat == m
        assert PKE.encrypt(pk, m, G(m)) == c

class CiphertextValidator:
    """CVO in game 0"""
    def check(pk, sk, c):
        m_hat = PKE.decrypt(sk, pk, c)
        assert PKE.encrypt(pk, m_hat, G(m)) == c
```

Game 0 is the IND-PCVA game, where the adversary has access to PCO and CVO as defined above.

Game 1 is identical to game 0, except that we modify the CVO to check the tape of the hash oracle $\mathcal{O}^G$ for the validity of the queried ciphertext:

```python
class CiphertextValidator:
    """CVO in game 1. Checks the validity of the ciphertext by reading the tape
    of the hash function G for matching queries. Notice that this CVO does not
    need the secret key
    """
    def check(pk, c, hash_g):
        for (m, r) in hash_g.tape:
            if PKE.encrypt(pk, m, r) == c:
                return True  # found matching hash query
        return False  # no matching hash query
```

game 1 differs from game 0 when the PCVA adversary makes CVO query that passes the re-encryption test (aka $\text{CVO}_0$ will return "valid") but has no matching query in the hash oracle's tape (aka $\text{CVO}_1$ will return "invalid"):

$$
P[S_0] - P[S_1] \leq q_V \cdot 2^{-\gamma}
$$

where $q_V$ is the number of ciphertext validation queries, and $\gamma$ is the spread of the underlying PKE.

Game 2 is identical to game 1, except that we modify the PCO to only check encryption, not decryption:

```python
class PlaintextChecker:
    """PCO in game 2. Checks the validity fo the ciphertext by only encrypting
    the message, and NOT checking the decryption of the ciphertext
    """
    def check(pk, m, c):
        for m_, r_ in hash_g.queries:
            if m_ == m and PKE.encrypt(pk, m_, r_) == c:
                return True  # is valid
        return False # is invalid
```

Game 2 differs from game 1 when the PCVA adversary makes a hash query $(\tilde{m}, \tilde{r})$ such that $\tilde{m}, \tilde{r}$ causes decryption error:

$$
P[S_1] - P[S_2] \leq q_G \cdot \delta
$$

where $q_G$ is the number of hash queries to G, and $\gamma$ the probability that a randomly chosen message causes decryption error.

Game 3 is identical to game 2, except when encrypting the challenge ciphertext, $r^\ast$ is truly random instead of being queried from $G(m^\ast_b)$. The behavior of the PCVA adversary differs between the two games if it ever queries $m^\ast$ from $G$.

$$
P[S_2] - P[S_3] \leq P[\text{query}]
$$

Notice that in game 3, PCO and CVO can both be simulated without the secret key, and the challenge ciphertext is encrypted with a truly random coin instead of a pseudorandom coin. This means an OW-CPA adversary can simulate the entire OW-PCVA game with only the public key and the challenge ciphertext:

$$
P[S_3] = P[\mathcal{A}_\text{OW-CPA}]
$$

Similarly, there is another OW-CPA adversary that simulates game 3, but checks the tape of hash $G$ for $m^\ast$:

$$
P[\mathcal{A}_\text{OW-CPA}] = P[\text{query}] \cdot \frac{1}{q_G}
$$

Putting everything together we first have:

$$
\epsilon_\text{OW-PCVA} = P[S_0] \leq q_V \cdot 2^{-\gamma} + q_G \cdot \delta 
    + P[\text{query}] + \epsilon_\text{OW-CPA}
$$

Substituting $P[\mathcal{A}_\text{OW-CPA}] = P[\text{query}] \cdot \frac{1}{q_G}$ in:

$$
\epsilon_\text{OW-PCVA} = P[S_0] \leq q_V \cdot 2^{-\gamma} + q_G \cdot \delta 
    + (q_G + 1) \cdot \epsilon_\text{OW-CPA}
$$

This is till not tight, but we can make two modifications:

- We can replace $\epsilon_\text{OW-CPA}$ by $\epsilon_\text{IND-CPA}$ using a "well-known result": $\epsilon_\text{OW-CPA} \le \epsilon_\text{IND-CPA} + \frac{1}{\vert\mathcal{M}\vert}$
- We can bound $P[\text{query}]$ using an IND-CPA game. The IND-CPA adversary checks if either of $(m_0, m_1)$ is on the tape of the hash oracle: if yes, then return the matching index; if not, return a random index

$$
P[\mathcal{A}_\text{IND-CPA}] = P[\text{query}] + \frac{1}{2}(1 - P[\text{query}])
$$

In other words, if the input PKE is IND-CPA (instead of OW-CPA), then we can achieve a tighter security bound:

$$
\epsilon_\text{OW-PCVA} = P[S_0] \leq q_V \cdot 2^{-\gamma} + q_G \cdot \delta 
    + 2 \epsilon_\text{IND-CPA}
    + \epsilon_\text{IND-CPA} + \frac{1}{\vert\mathcal{M}\vert}
$$


## IND-CCA KEM
The custom PCVA security definition is useful because given a OW-PCVA PKE, we can make an IND-CCA KEM. The transformation is as follows:

```python
class KEM:
    """KEM transformation, with explicit rejection, and probabilistic PKE
    """
    def keygen():
        return PKE.keygen()

    def encapsulate(pk):
        m = PKE.sample_message()
        ct = PKE.encrypt(pk, m, G(m))
        ss = H(m, ct)
        return ct, ss
    
    def decapsulate(sk, ct):
        m = PKE.decrypt(sk, ct)
        if PKE.encrypt(pk, m, G(m)) != ct:
            raise InvalidCiphertextError
        return H(m, ct)
```

Game 0: the KEM IND-CCA game. Hash functions $H, G$ are both random oracles, and the adversary has access to a decapsulation oracle $\mathcal{O}^D$.

Game 1 is identical to game 0, except we modify both the hash function $H$ and the decapsulation oracle so that they can operate without the secret key. Instead, they make use of PCO and CVO, which later on will allow the game to be simulated by a OW-PCVA adversary.

- $\mathcal{O}^D$ now maintains a tape that records the query $(\tilde{c}, \tilde{K})$, where $\tilde{c}$ is the input ciphertext and $\tilde{K}$ is the output shared secret
- When $H$ handles a hash query $(\tilde{m}, \tilde{c})$, it queries the PCO. In addition to returning the correct hash value $\tilde{K}$ (be it fresh or not), if the PCO confirms that the query is a valid pair, $H$ will add $(\tilde{c}, \tilde{K})$ to the tape of $\mathcal{O}^D$
- When $\mathcal{O}^D$ handles a query $\tilde{c}$, it first checks CVO for ciphertext validity: invalid ciphertext query will return a rejection. For valid ciphertext, $\mathcal{O}^D$ checks its own tape. If $\tilde{c}$ is in the tape, the matching $\tilde{K}$ will be returned; otherwise, a truly random $\tilde{K}$ is sampled, returned, and $(\tilde{C}, \tilde{K})$ is recorded to the tape

We claim that from the IND-CCA adversary's point of view, game 1 is identical to game 0: if the query is invalid, both decapsulation queries will reject; if the query is valid, then $\mathcal{O}^D$ will correctly return $H(\tilde{m}, \tilde{c})$.

Game 2 is identical to game 0, except that we further modify the hash function $H$: if the hash query $(\tilde{m}, \tilde{c})$ is identical to the challenge plaintext/ciphertext pair $(m^\ast, c^\ast)$, then $H$ will return a second truly random value $\tilde{K}$ instead of the value used for challenge encryption.

Game 2 differs from game 1 if the IND-CCA adversary ever makes a hash query $(\tilde{m}, \tilde{c})$ such that $\tilde{m} = m^\ast$:

$$
P[S_1] - P[S_2] \leq P[\text{query}]
$$

First, we claim that in game 2, the challenge values $(c^\ast, K^\ast)$ are correlated only through the hash function $K^\ast = H(m^\ast, c^\ast)$ (for when $K^\ast$ is pseudorandom). If the IND-CCA adversary will only get a truly random value when querying $H(m^\ast, c^\ast)$, then it receives absolutely no information about the challenge values, and there can be no advantage: $P[S_2] = \frac{1}{2}$.

Second, the probability of making the query can be bounded by the advantage of an OW-PCVA adversary. For one, game 2 can be perfectly simulated by the OW-PCVA adversary: $H$, $\mathcal{O}^D$ can both be simulated using tapes, PCO, and CVO; when performing the KEM challenge encapsulation, the KEM ciphertext is the OW-PCVA challenge ciphertext, and $K^\ast$ is truly random. After the IND-CCA adversary halts, the OW-PCVA adversary can check the tape of $H$ for any query $(\tilde{m}, \tilde{c})$ such that $\tilde{c} = c^\ast$, and returns the matching $\tilde{m}$ as the guess:

$$
P[\mathcal{A}_\text{OW-PCVA}] = P[\text{query}]
$$
