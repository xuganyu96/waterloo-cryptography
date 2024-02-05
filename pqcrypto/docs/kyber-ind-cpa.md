IND-CCA2 security for Kyber is achieved by applying (a tweaked version of) Fujiaski-Okamoto transformation to an IND-CPA secure version of Kyber. As a result, much of the IND-CCA2 keygen, encryption, and decryption routines are based on IND-CPA implementation, so we will understand the IND-CPA implementation before moving onto the Fujisaki-Okamoto transformation.

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

The public key's size is `KYBER_K * KYBER_POLYBYTES + KYBER_SYMBYTES` where `KYBER_POLYBYTES` is always 384 (256 coefficients each taking 12 bits to encode) and `KYBER_SYMBYTES` is always 32. This means that the public key contains the seed for the matrix $A \in R_q^{k \times k}$ and the actual values for $\mathbf{b} \in R_q^k$.

## Centered binomial distribution
Consider the random variable that is the difference between two fair coin tosses $X = I_1 - I_2$. The probability of getting $\pm 1$ is $\frac{1}{4}$ and the probability of getting 0 is $\frac{1}{2}$.

This means that $X$ actually follows the centered binomial distribution $\mathcal{B}(n=2, p=\frac{1}{2})$. As a result, the sum of $\eta$ of i.i.d. of $X_i = I_{i, 1} - I_{i, 2}$ follows the centered binomial distribution $\mathcal{B}(n=2\eta, p=\frac{1}{2})$. This is the basis on which Kyber samples from the desired centered binomial distribution

But does it generalize to any $p$?

## Generating the sample matrix
The LWE matrix $A$ in the public key is generated from a 32-byte seed. The value of the seed itself is randomly derived (in the reference implementation it is derived from `/dev/urandom`).

```rust
/// Snippet for reading from /dev/urandom
use std::io::Read;
use std::fs::File;

let mut fd = File::open("/dev/urandom").unwrap();
let mut randombytes = [0u8; 32];
let _ = fd.read(&mut randombytes);
println!("{randombytes:?}");
```

The sample matrix $A \in R_q^{k_2 \times k_1}$ (for all Kyber $R_q = \mathbb{Z}_q[x] / \langle x^{256} + 1 \rangle$, for Kyber-512 $k_2 = k_1 = 2$) is generated in `indcpa.c::gen_matrix`. The randomness is derived principally derived from a 32-byte seed value passed in from the caller, but for each entry polynomial in the matrix $A$, the index $(i, j)$ is also used to initialize the Keccak state.

Each entry polynomial of the matrix is generated independently from the Keccak output. It is worth noting that the `poly` type is used both for coefficient representation and for NTT representation. When used in coefficient representation, the values in `poly.coeffs` should fall within $[-1664, 1664]$. On the other hand, under NTT representation, a non-negative residue is used. This can be seen in `rej_uniform` where `val0`, `val1` both have `uint16_t` and are rejected based on `KYBER_Q` instead of `KYBER_Q / 2`.

> Coefficient domain's values falls within $[-1664, 1664]$
> NTT domain's values falls within $[0, 3329)$

In other words, when `poly` type is used to encode NTT representation, the values need to be cast into `uint16_t` first before the representation makes sense.

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