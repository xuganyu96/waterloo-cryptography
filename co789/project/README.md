# Post-quantum secure-channel protocols
For the CO789 project I want to survey the application of post-quantum cryptography in secure-channel protocols. TLS is the most prominent target, but SSH is also a good target.

Targets:
- TLS
- SSH
- WireGuard

What's next:

- [Post-quantum TLS without handshake signature](./post-quantum-tls-without-signature.pdf)
- Look for Google's CECPQ1/CEC PQ2, Cloudflare's X25519/NTRU-HRSS.

# Authenticated key exchange using KEM
Post-quantum KEMs are usually faster and more compact than post-quantum digital signatures. Under appropriate context it might be desirable to design a secure channel that is established that does not rely on post-quantum digital signatures.

## State of TLS 1.3
There are two primary goals:
1. Client and server need to establish a shared secret
2. Server needs to authenticate itself to the client

Goal 1 is achieved using a key exchange (`ClientHello` and `ServerHello`). In the case of TLS 1.3, the chosen key exchange algorithms include finite-field and elliptic-curve Diffie-Hellman.

Goal 2 is achieved using digital signatures (`CertificateRequest` and `Certificate`). With TLS, the client has a list of Certificate Authority's public keys (for signature verification), and the server authenticates itself by producing the server's public key, which itself is signed by the Certificate authority. The collection of server's public key and CA's signature on server's public key binds the public key to the 