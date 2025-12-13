# Secure implementations
I want to learn to implement cryptographic algorithms with focus on **correctness, security, and speed**.

Start with implementing well-known crypto schemes:
- SHA-2, AES, ChaCha20-Poly1305
- Big integer, RSA, X25519, Ed25519
- ML-KEM, including KyberSlash and Clangover

I also want to publish. Secure implementation of PQC schemes is an active area of research, but I am not ready to contribute.
Instead, I should use classical schemes to learn low-level fundamentals: compiler behavior, LLVM and intermediate languages, etc.
Given that [support constant-time code in LLVM](https://discourse.llvm.org/t/rfc-constant-time-coding-support/87781/29) is under active consideration, learning from established schemes might even produce some good results.
