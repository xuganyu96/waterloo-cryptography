# What's next
**Code-based cryptography**: the current batch of standardized post-quantum cryptography algorithms (ML-KEM, ML-DSA, STH-DSA, and Falcon, which was expected to be standardized soon) concentrated too heavily on hard lattice problems. The 4th round of KEM proposals will likely see a winner using another category of hard problems. Cryptography based on error-correcting code, especially Classic McEliece, is particularly promising because of its stable security levels throughout the past many decades (whereas attacks on lattice-based cryptography have improved by a lot in the same period of time).

For the next project, I will likely be working on some improvements to classic McEliece. Error-correcting code is also an immensely interesting field of studying that I want to know more about. Potential side projects include:

- Some implementation of a Reed-Solomon code
- Alternative implementation of classic McEliece: there seems to be a lack of pure Rust implementation (the best I can find is [this](https://crates.io/crates/classic-mceliece-rust) but it was updated 2 years ago)