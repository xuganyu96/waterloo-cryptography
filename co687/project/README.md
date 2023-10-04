## Introduction
Learning with errors (LWE) and related constructions have been a rich source of computational problems that have robust reduction to known hard problems in lattices and that have been applied to a large variety of cryptographic constructions, including collision-resistant hashing, digital signatures, key exchange, and public-key/symmetric-key cryptosystems with nice properties (e.g. FHE).

Unfortunately, there is a genuine lack of introductory material to bring undergraduate-level students and engineers up to speed with the current state of the art and to provide mathematical intuition behind the various cryptographic applications. Students and engineers with limited mathematical background (such as the author himself) finds the jump from "what is LWE" to "building meaningful things in cryptography" rather daunting and unapprochable.

For this project, this article aims to provide a gentle introduction to the mathematics behind LWE and expand in two directions:

1. From LWE to hard lattice problems, most notably Regev's results linking the hardness of LWE to the hardness of shortest vector problem
2. From LWE to application in cryptography, including
    - Collision resistant hashing
    - Digital signature through lattice trapdoors
    - Optimization through polynomial ring

Time and effort permitting, the author also plans to explore advanced/adjacent topics including:

1. `NTRUEncrypt` and `NTRUSign`
2. Fully homomorphic encryption

**Table of content**

- Background information in linear algebra and probability
- Statement of LWE and some equivalent variations
- **reduction of LWE to SVP**
- Collision resistant hashing
- Lattice trapdoor and digital signature
- Public-key/symmetric-key cryptosystem
- (Bonus) NTRU
- (Bonus) fully homomorphic encryption


## Resources
- [On lattices, learning with errors, random linear codes, and cryptography](https://cims.nyu.edu/~regev/papers/qcrypto.pdf)
- [Practical Lattice-based Digital Signature Schemes](https://csrc.nist.gov/csrc/media/events/workshop-on-cybersecurity-in-a-post-quantum-world/documents/papers/session9-oneill-paper.pdf)