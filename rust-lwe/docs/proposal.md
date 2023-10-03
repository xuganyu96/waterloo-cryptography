# CO 685/687 Project Proposal

## Introduction
- Lattice is a great candidate because it has been extensively studied and provide a solid basis of hard problems that can be used to build cryptographic primitives and crypto systems
- We start with statement of the classical SIS and LWE problems and discuss some theoretical cryptographic constructions based on the classical SIS/LWE formulation
- Cryptographic constructions based on classical formulation of SIS/LWE have inefficient key and ciphertext sizes; discuss how to use polynomial ring to formulate alternative statement (ring SIS and ring LWE), and how they are reduce key sizes and/or ciphertext sizes
- Implement some simple primitives
    - Collision resistant hashing using SIS
    - Digital dignature using a lattice trapdoor
    - Collision resistant hashing using ring SIS

This project will largely be based on the lectures from [MIT's CS294](https://people.csail.mit.edu/vinodv/CS294/), particularly:

- lecture 1: statement of classic SIS/LWE and their variants, simple hash function, symmetric key system, and public key system
- lecture 2-4: attacks on LWE, reduction of LWE/SIS to hard lattice problems
- lecture 6: trapdoor for solving LWE and SIS, digital signature scheme from such trapdoor
- lecture 10: ring SIS, ring LWE, NTRU

I think it's a realistic goal to
- Understand and explain the reduction arguments
    - From ring to classical
    - From SIS/LWE to hard lattice problems
- Understand and explain simple cryptographic constructions
    - Formulation: KeyGen, Encrypt, Decrypt
    - Correctness argument
    - Security argument
- Implement a hash function and a digital signature scheme based on ring-SIS or ring-LWE and state some performance characteristics and stuff


## Resources, references
- [Practical Lattice-based Digital Signature Schemes](https://csrc.nist.gov/csrc/media/events/workshop-on-cybersecurity-in-a-post-quantum-world/documents/papers/session9-oneill-paper.pdf)