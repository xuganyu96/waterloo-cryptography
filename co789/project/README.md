# Post-quantum secure-channel protocols
For the CO789 project I want to survey the application of post-quantum cryptography in secure-channel protocols. TLS is the most prominent target, but SSH is also a good target.

What's next:
- [ ] Fork `rustls` for setting up a TLS server that can perform a handshake, then simply echo all application data
- [ ] Incorporate PQ cryptographic primitives into `rustls` using [`liboqs-rust`](https://github.com/open-quantum-safe/liboqs-rust)
- [ ] Implement KEMTLS in rustls?

# Current state of post-quantum TLS on embedded systems

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