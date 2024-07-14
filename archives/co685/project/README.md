# Benaloh crypto system
For CO 685's project I want to do an implementation of the [Benaloh cryptosystem](https://en.wikipedia.org/wiki/Benaloh_cryptosystem) in Rust.

Goals:
- [ ] Ambient prime generation $p, q, r$
- [ ] Key generation: $y, x, n$
- [ ] Encrypt and decrypt, including tests for correctness
- [ ] Demonstrate ciphertext homomorphism
- [ ] Extend into an electronic election scheme

## References:
- [A robust and verifiable cryptographically secure election scheme (1985)](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/11/elect.pdf)
- [Verifiable secret-ballot elections (1996)](https://www.microsoft.com/en-us/research/wp-content/uploads/1987/01/thesis.pdf)
- [A Java implementation](https://github.com/nasimmaleki/Cryptography)
- [A Go implementation](https://github.com/mirzazhar/benaloh)