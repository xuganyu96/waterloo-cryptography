# A gentle introduction to LWE

## Introduction
Learning with errors (LWE) and related constructions have been a rich source of computational problems that provide a robust and versatile foundation enabling a large variety of cryptographic applciations, including collision-resistant hashing, digital signatures, key exchange, and public-key/symmetric-key cryptosystems with nice properties (e.g. FHE).

The last decade saw a substantial increase of interest in lattice-based cryptography, but unfortunately there is a genuine lack of introductory material. At this moment, high quality content are concentrated in surveys that assume graduate level of mathematical background and lecture notes that are more complementary to the lecturers than comprehensive. This survey aims to bridge the jump from *"what is LWE"* to *"building meaningful crypto stuff"* by reviewing a select number of topics:

- What is LWE: definition and intuition, variants and their equivalences
- Why LWE: reduction to hard lattice problems
    - [Regev '09](https://cims.nyu.edu/~regev/papers/qcrypto.pdf)
- How to apply LWE: a collection of cryptographic applications
    - Collision resistant hashing
    - Digital signature
    - Public-key and symmetric-key cryptosystem
    - Ring variant of LWE
- Bonus material if time permits
    - NTRU
    - Fully homomorphic encryption


## Resources
- [On lattices, learning with errors, random linear codes, and cryptography](https://cims.nyu.edu/~regev/papers/qcrypto.pdf)
- [Practical Lattice-based Digital Signature Schemes](https://csrc.nist.gov/csrc/media/events/workshop-on-cybersecurity-in-a-post-quantum-world/documents/papers/session9-oneill-paper.pdf)
- [An intense introduction to cryptography](https://intensecrypto.org/public/index.html)  
Online textbook written by Barak Boaz, includes a chapter on lattice-based cryptography
- [A decade of lattice cryptography](https://web.eecs.umich.edu/~cpeikert/pubs/lattice-survey.pdf)  
A survey by Chris Peikert
- [Implementation of Lattice-based Signature scheme ring-TESLA and comparison with ECSDA](https://crypto.stanford.edu/cs359c/17sp/projects/KennethXu.pdf)
- [TESLA-sharp](https://eprint.iacr.org/2016/1026.pdf)
- [qTESLA](https://eprint.iacr.org/2019/085.pdf)