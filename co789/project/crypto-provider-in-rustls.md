# Crypto provider in `rustls`
The [example](https://github.com/rustls/rustls/blob/main/provider-example/src/lib.rs) provided by `rustls` for illustrating the implementation of an alternative crypto provider defines the alternative `CryptoProvider` struct behind a function call:

```rust
pub fn provider() -> CryptoProvider {
    CryptoProvider {
        cipher_suites: ...,
        kx_groups: ...,
        signature_verification_algorithms: ...,
        secure_random: ...,
        key_provider: ...,
    }
}
```

There are five fields to fill out, among which `secure_random` (referring to a cryptographically strong randomness generator) is the easiest to fulfill.

The field `cipher_suites` is easy to fill out in TLS 1.3 because this version of TLS has significantly cleaned up the permitted set of ciphers that can be used, and `ring` already implemented them:
1. ChaCha20-Poly1305-SHA256
2. AES128-GCM-SHA256
3. AES256-GCM-SHA384

The field `kx_groups` takes a vector of **references to structs that implement `SupportedKxGroup` and that have a `static` lifetime**. In addition, `SupportedKxGroup` requires implementation of `start`, which returns `Box<dyn ActiveKeyExchange>`, so we also need to implement a struct that implements `ActiveKeyExchange`.

An [example key exchange](https://github.com/rustls/rustls/blob/main/provider-example/src/kx.rs) was provided by `rustls`, although with ECDHE/FFDHE the routine is identical between the client and the server: both run the "keygen" routine to get a random $x$ from the chosen group, computes $g^x$ using $g$ from the chosen group, emits $g^x$, then uses peer's public key and self's ephemeral secret to arrive at the same shared secret. On the other hand, Kyber as a KEM will require the client and the server to perform different routines for establishing a shared secret.

One way to adapt `SupportedKxGroup` and `ActiveKeyExchange` is to implement two sets of structs for client and for server...

- Client calls `SupportedKxGroup::start()` to generate key pair
- Client calls `ActiveKeyExchange::pub_key()` to emit the public key
- Server calls `SupportedKxGroup::start()` to generate the shared secret
- Server calls `ActiveKeyExchange::complete()` on client's public key to encapsulate the shared secret
- Server calls `ActiveKeyExchange::pub_key()` to emit the shared secret
- Client calls `ActiveKeyExchange::complete()` on server's encapsulated secret to recover the secret

Unfortunately, in the default `rustls` server implementation, public key is queried before `ActiveKeyExchange::complete()` is called. This is inconvenient because the server needs to first receive the client's public key in `complete` before it can emit the encapsulated secret.

**TODO: check how hard it is to swap the order of the call**
- public key is queried in `rustls/server/tls13.rs` under `emit_server_hello`
- As for where `complete` is called
    - Called by `ActiveKeyExchange::complete_for_tls_version`
    - Called by `rustls/crypto/rls13.rs` under the trait `Hkdf::extract_from_kx_shared_secret`
    - Called by `rustls/tls13/key_schedule.rs` under the struct `KeySchedule`
    - Called by `rustls/tls13/key_schedule.rs` under the struct `KeySchedulePreHandshake::into_handshake`
    - Called by `emit_server_hello`

Post-quantum hybrid KEX: https://datatracker.ietf.org/doc/draft-tls-westerbaan-xyber768d00/