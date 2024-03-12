# Post-quantum secure-channel protocols
For the CO789 project I want to survey the application of post-quantum cryptography in secure-channel protocols. TLS is the most prominent target, but SSH is also a good target.

What's next:
- [ ] Fork `rustls` for setting up a TLS server that can perform a handshake, then simply echo all application data
- [ ] Incorporate PQ cryptographic primitives into `rustls` using [`liboqs-rust`](https://github.com/open-quantum-safe/liboqs-rust)
- [ ] Implement KEMTLS in rustls?

# [Benchmarking post-quantum cryptography in TLS](./papers/benchmarking-post-quantum-cryptography-in-tls.pdf)
Thanks to the forward compatible designsof TLS 1.3, adopting post-quantum cryptography is straightforward. 

On the key exchange front, the client adds post-quantum key encapsulation algorithm identifier (e.g. ML-KEM) in the `supported_groups` extension, and the post-quantum public key in the `key_share` extension. There currently exists an Internet Draft for [`X25519Kyber768Draft00`](https://datatracker.ietf.org/doc/draft-tls-westerbaan-xyber768d00/) that proposes using a hybrid key-exchange, where the public key material contains the concatenation of an ECDH public key and a ML-KEM public key. This hybrid key exchange trades increased communication sizes and cryptographic operations for compliance and the proven security of ECDH against classical adversaries.

P.S. personal experience working with the Rust TLS library `rustls`, the logic for establishing shared secret was implemented specifically for DH-style key exchange. For example, the server sends its `key_share` BEFORE processing the client's `key_share`, which works in a DH setting since the order of execution does not matter, but it does not work in a KEM setting because the server must process client's `key_share` (by running `KEM.encapsulate`) before it makes sense to transmit the server's `key_share`

On the authentication front, there are two places where post-quantum signatures are used in TLS 1.3: `signature_algorithms_cert` and `signature_algorithms` extensions in the `ClientHello` message. `signature_algorithms_cert` indicates which signature algos can be used in X.509 Certificates. `signature_algorithms` indicates which signatures can be used in the `CertificateVerify` message.

When server authenticates itself to the client (the reverse could also happen when client authentication is requested), the server presents a X.509 Certificate, which contains some metadata (server name, domain names, date of expiration, etc.), **the server's public key**, and a **signature on the server's public key by some trust anchor** (typically a certificate authority such as Let's Encrypt, although X.509 also supports a chain of trust, which is not the focus of this discussion). In addition to presenting a valid X.509 certificate, the server also needs to demonstrate its posession of the matching secret key, which is done by signing a transcript of the handshake so far and transmitting it in the `CertificateVerify` message. Both the `Certificate` and the `CertificateVerify` are required for authentication, or the peer should abort the handshake.

## Evaluating the choice of post-quantum algorithms
On server-grade hardware, ROM and RAM sizes pose negligible constraint, so the main considerations are the time cost of public-key cryptography operations and the communication sizes.

This paper presents a practical setup for testing the performance (mostly in handshake completion time) of incorporating post-quantum primitives into TLS using hybrid key exchange and post-quantum-only authentication. **Where network is fast and reliable, public key cryptographic operations are the dominant factors in handshake time; where network latency is high and/or packet loss is at 3%-5% or higher, the communication size becomes the dominant factors**, so engineering decisions should be made with real-world network and computing hardware in mind.

# An introduction of TLS 1.3?

# Additional papers to read through

- [(2021) Some summary of incorporating PQC in embedded systems](https://csrc.nist.gov/CSRC/media/Events/third-pqc-standardization-conference/documents/accepted-papers/atkins-requirements-pqc-iot-pqc2021.pdf)  
RAM is a major constraint. Dilithium has enormous memory footprint that cannot fit onto some RAM-constrained environments. Falcon has a smaller memory footprint
- [(2022) Performance evaluation of post-quantum TLS 1.3 on resource-constrained embedded systems](https://link.springer.com/chapter/10.1007/978-3-031-21280-2_24)  
Use WolfSSL with modification to use PQC KEM and Digital signature in replacement, measured communication sizes, timing, etc.
- [Post-quantum TLS without handshake signature](./post-quantum-tls-without-signature.pdf)  
Authenticate using KEM instead of digital signature
    - [Other people's implementation of KEMTLS](https://link.springer.com/chapter/10.1007/978-3-031-22829-2_6)  
    Where network bandwidth is more constrained, choose PQC with smaller signature/public-key/ciphertext size at the cost of more complex computation; where network bandwidth is not constrained, choose PQC with larger signature/pk/ciphertext sizes but faster computation.
    - What if public key is already installed in the client? Then public-key size does not need to be transmitted across the network
    - What if the client needs to be authenticated? This might be important because Falcon's `verify` is fast but `sign` is slow
- Look for Google's CECPQ1/CEC PQ2, Cloudflare's X25519/NTRU-HRSS.