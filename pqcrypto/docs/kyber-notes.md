---
title: Kyber's reference implementation
---

- [Appendix](#appendix)
    - [Using a debugger](#a-using-a-debugger)

# Generating key pair
The IND-CCA2 key generation 

These are the steps of `indcpa_key` from `indcpa.c`:
- `gen_a`
    - `gen_a(A, B)` is an alias for calling `gen_matrix(A, B, 0)`, which is to generate the matrix $A$ without transposing
    - With the reference (modern) implementation of Kyber, Keccak is used for the XOF (extensible output function). Keccak follows the sponge design, where `xof_absorb` takes possibly random inputs to update the internal state, and `xof_squeezeblocks` outputs outputs cryptographic-strength random bits
- `poly_getnoise_eta1`
- `polyvec_ntt`
- `polyvec_basemul_acc_motgomery`
- `polytomont`
- `polyvec_add`
- `polyvec_reduce`
- `pack_sk`
- `pack_pk`

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