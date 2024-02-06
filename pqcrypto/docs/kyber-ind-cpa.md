IND-CCA2 security for Kyber is achieved by applying (a tweaked version of) Fujiaski-Okamoto transformation to an IND-CPA secure version of Kyber. As a result, much of the IND-CCA2 keygen, encryption, and decryption routines are based on IND-CPA implementation, so we will understand the IND-CPA implementation before moving onto the Fujisaki-Okamoto transformation:

```c
/**
 * indcpa.c: line 205
 */

void indcpa_keypair(uint8_t pk[KYBER_INDCPA_PUBLICKEYBYTES],
                    uint8_t sk[KYBER_INDCPA_SECRETKEYBYTES])
{
    unsigned int i;
    uint8_t buf[2*KYBER_SYMBYTES];
    const uint8_t *publicseed = buf;
    const uint8_t *noiseseed = buf+KYBER_SYMBYTES;
    uint8_t nonce = 0;
    polyvec a[KYBER_K], e, pkpv, skpv;

    randombytes(buf, KYBER_SYMBYTES);
    hash_g(buf, buf, KYBER_SYMBYTES);

    gen_a(a, publicseed);

    for(i=0;i<KYBER_K;i++)
        poly_getnoise_eta1(&skpv.vec[i], noiseseed, nonce++);
    for(i=0;i<KYBER_K;i++)
        poly_getnoise_eta1(&e.vec[i], noiseseed, nonce++);

    polyvec_ntt(&skpv);
    polyvec_ntt(&e);

    // matrix-vector multiplication
    for(i=0;i<KYBER_K;i++) {
        polyvec_basemul_acc_montgomery(&pkpv.vec[i], &a[i], &skpv);
        poly_tomont(&pkpv.vec[i]);
    }

    polyvec_add(&pkpv, &pkpv, &e);
    polyvec_reduce(&pkpv);

    pack_sk(sk, &skpv);
    pack_pk(pk, &pkpv, publicseed);
}

```

## Generating seeds
For IND-CPA keygen, two 32-byte seeds are used for generating the public matrix $A$ and the secret/error vectors respectively (note that the secret vector and the error vector are generated using the same seed, but with different nonce).

In `indcpa_keypair`, 32 bytes of randomness is first extracted from `/dev/urandom`, then fed into SHA3-512 to expand into 64 bytes of hash value, where the first half is used for generating $A$ and the second half used to generate $\mathbf{s}$ and $\mathbf{e}$.

Using Python's `hashlib` it can be confirmed that `hash_g`, in the instantiation of Kyber-512, is SHA3-512:

```python
from hashlib import sha3_512
captured_ios = [
    (
        "2cf85ad21abde69bac39050ce255ad97325693cc37c840ba9c1ff3176ee37a34",
        "876f8d3100f58abfbc4e145598d60c4d69940da68598b807b88bec36464e44e6"\
        "8dd8674c4f2e49364eafd2aee3e08f9eafffa92bc321dd308753406de74f138e"
    ),
    (
        "e4c0637be7e50ffdc8d62e16a5652870e362bf652dd8eed3db8095738ddc97fa",
        "4960858da2496921c1f61279d41071b977282c707d3285cffdf310da249f1f7f"\
        "33fa9789d865bbf494287e24931fab862aa3837a37c6d672afc6c926b3735aa7"
    ),
    (
        "a106dfebe2c5de127e6d831c04b221064f41839b7b1bf6ef022ce8acd57724ad",
        "687141e50e8c5b981a945830d456448dd527bea13afed0faf50056d8f1143e1c"\
        "35f8ec393394a91d41808fe11c52ed3be59705bcd030ba25d1b029cf7bc87357"
    )
]

for data, expected_digest in captured_ios:
    hash = sha3_512()
    hash.update(bytes.fromhex(data))
    assert hash.hexdigest() == expected_digest
```

TODO: Rust implementation pending

## Sampling uniformly random matrix 
```c
void gen_matrix(polyvec *a, const uint8_t seed[KYBER_SYMBYTES], int transposed)
{
  unsigned int ctr, i, j, k;
  unsigned int buflen, off;
  uint8_t buf[GEN_MATRIX_NBLOCKS*XOF_BLOCKBYTES+2];
  xof_state state;

  for(i=0;i<KYBER_K;i++) {
    for(j=0;j<KYBER_K;j++) {
      if(transposed)
        xof_absorb(&state, seed, i, j);
      else
        xof_absorb(&state, seed, j, i);

      xof_squeezeblocks(buf, GEN_MATRIX_NBLOCKS, &state);
      buflen = GEN_MATRIX_NBLOCKS*XOF_BLOCKBYTES;
      ctr = rej_uniform(a[i].vec[j].coeffs, KYBER_N, buf, buflen);

      while(ctr < KYBER_N) {
        off = buflen % 3;
        for(k = 0; k < off; k++)
          buf[k] = buf[buflen - off + k];
        xof_squeezeblocks(buf + off, 1, &state);
        buflen = off + XOF_BLOCKBYTES;
        ctr += rej_uniform(a[i].vec[j].coeffs + ctr, KYBER_N - ctr, buf, buflen);
      }
    }
  }
}
```

The data type for the public matirx $A \in R_q^{k_2 \times k_1}$ is an array of `polyvec`, where each `polyvec` contains an array of `poly`, and each `poly` contains an array of `KYBER_N` 16-bit integers.

```rust
const KYBER_N: usize = 256;
const KYBER_K: usize = 2;

pub struct Poly {
    coeffs: [u16; KYBER_N],
}

pub struct PolyVec {
    poly: [Poly; KYBER_K],
}
```

Uniformly random bytes are derived using Shake128, where the input consists of the concatenation of the [public seed](#generating-seeds) and the indices `x, y`. 

```c
/** symmetric-shake.c */
void kyber_shake128_absorb(keccak_state *state,
                           const uint8_t seed[KYBER_SYMBYTES],
                           uint8_t x,
                           uint8_t y)
{
  uint8_t extseed[KYBER_SYMBYTES+2];

  memcpy(extseed, seed, KYBER_SYMBYTES);
  extseed[KYBER_SYMBYTES+0] = x;
  extseed[KYBER_SYMBYTES+1] = y;

  shake128_absorb_once(state, extseed, sizeof(extseed));
}
```

Uniformly random elements of $\mathbb{Z}_q$ are obtained from rejection sampling `rej_uniform`. In Kyber's implementation, because elements in $\mathbb{Z}_{3329}$ can be encoded using 12 bits, the sampling methods will obtain (up to ) two random samples per three truly random bytes, which reduces the amount of randomness needed to sample the required number of elements. 

In the context of sampling random elements modulus 3329, "rejection" sampling means that if the random 12 bits encode an integer that is not less than 3329, then this integer is "rejected". Therefore, there is a chance that more than $3n$ bytes are needed to generate $n$ random elements. Kyber's implementation first makes an attempt to fill all 256 coefficients with random samples, but in the unfortunate situation where the initial amount of randomness (squeezed into `buf`) did not produce enough samples in $\mathbb{Z}_{3329}$, additional randomness will be squeezed into the front of `buf` for more sampling untill all 256 numbers are filled.

## Public key and secret key types
The output of `indcpa.c::indcpa_keypair` is the key pair: public key and secret key.

```rust
const SEED_BYTES: usize = 32;

/// Data type for the public key
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct KyberPublicKey<const K: usize> {
    seed: [u8; SEED_BYTES],

    /// The in-memory representation will not be encoded
    b: [Poly; K],
}

impl<const K: usize> KyberPublicKey {
    pub fn write_to_bytes(&self, pk_bytes: &mut [u8]) {
        assert_eq_debug!(pk_bytes.len() == K * Poly::BYTES + SEED_BYTES);

        self.b.iter()
            .for_each(|poly| {
                // Write poly's byte representation to pk_bytes
            });
        // Write SEED_BYTES to pk_bytes
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct KyberSecretKey<const K: usize> {
    secret: [Poly; K],
}
```






This means that $X$ actually follows the centered binomial distribution $\mathcal{B}(n=2, p=\frac{1}{2})$. As a result, the sum of $\eta$ of i.i.d. of $X_i = I_{i, 1} - I_{i, 2}$ follows the centered binomial distribution $\mathcal{B}(n=2\eta, p=\frac{1}{2})$. This is the basis on which Kyber samples from the desired centered binomial distribution

But does it generalize to any $p$?

## Compression
Ciphertext compression is implemented in [this way](https://github.com/pq-crystals/kyber/commit/272125f) because the original implementation contains division on secret information by `KYBER_Q` that leaves open a timing vulnerability.

# Appendix
## A: Using a debugger
For how complex the reference implementation is, using a debugger is tremendously helpful. MacOS' developer toolchain comes with `lldb` out of the box, so we will use that.

Here are the things I usually do using a debugger (where `(lldb)` is in parenthesis, the command is executed within the debugger; otherwise the command is executed on the shell):

|what I want to do|command|
|:--|:--|
|Load a program|`lldb foo`|
|Start the program|`(lldb) run`|
|Breakpoint at line|`(lldb) breakpoint set --file <filepath> --line <line_no>`|
|Breakpoint at function|`(lldb) breakpoint set --name <myfunc>`|
|Continue exection|`(lldb) c(ontinue)`|
|Execute until the next line|`(lldb) n(ext)`|
|Step into|`(lldb) s(tep)`|
|Step out|`(lldb) f(inish)`|
|List all variables|`frame variable [var_name] --format [hex,bin,dec,oct]`|
|Summary of the current frame|`frame info`|
|Exit the program|???|