# Public-key cryptography in TLS 1.3
Where is public-key cryptography applied in the TLS 1.3 protocol? Key exchange and authentication.

In `ClientHello`:
- `supported_groups` indicate the key exchange methods that the client supports, 
- `supported_signature_algorithms` indicate the signatures that the client supports
- `key_share` contains an ephemeral ECDH/RSA public key

In `ServerHello`:
- `key_share` contains server's ephemeral public key

After `ServerHello`, all messages will be encrypted (but with what?) From here:

1. Server sends `EncryptedExtension`
2. Server sends `CertificateRequest` (if client needs to be authenticated)
3. Server sends `Certificate`
4. Server sends `CertificateVerify` (a digital signature over a hash of all exchanged handshake messages so far)
5. Server sends `ServerHandshakeFinished`
6. Client sends `Certificate`, `CertificateVerify`, and `ClientHandshakeFinished`
7. After the handshake is finished, client and server sends application data that are protected using symmetric ciphers, which are largely unaffected by quantum computers.

## Modification from [Tasopoulos et al](./performance-evaluation-of-post-quantum-tls13.pdf)
- Added named groups for `supported_groups` in `ClientHello`, which is for key-exchange
- Added supported signatures in `supported_signature_algorithms` in `ClientHello`; this signature is for `CertificateVerify`
- ECDH key exchange is replaced with the KEM provided by Kyber: client generates key pair and sends the public key to the server in `ClientHello`; server sends back the encapsulated secret in `ServerHello`. The shared secret is passed to KDF to produce master secret and intermediate secrets
- X.509 Certificates now need to be signed with post-quantum signatures

Where is `supported_groups` in the `ClientHello`?
- In `rustls:ClientConfig`
- In `rustls::ClientConnection::new`

in `rustls::ClientConfig`, `provider.kx_groups` includes:
- `X25519`
- `secp256r1`
- `secp384r1`

This seems to be the set of supported groups.

`rustls::client::hs::emit_client_hello_for_retry` contains the high-level code that specified `supported_groups`:

```rust
let mut exts = vec![
    ClientExtension::NamedGroups(
        config
            .provider
            .kx_groups
            .iter()
            .map(|skxg| skxg.name())
            .collect(),
    ),
    // ...
];
```

Thanks to the abstraction of `rustls`'s cryptographic backend, adding `NamedGroups` is now handled by user's implementation of a `CryptoProvider`.