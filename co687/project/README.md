# What's next

- [ ] Read up on [LLL basis reduction](https://cseweb.ucsd.edu/classes/fa21/cse206A-a/LecLLL.pdf) and write it into the preliminaries
- [ ] Read up on [dual lattice](https://cseweb.ucsd.edu/classes/fa21/cse206A-a/Lec3-Dual.pdf) and its relationship for CVP
- [ ] Read up on Hermite normal form and write it into the preliminaries
- [ ] Write a section on SIS and how it is related to lattices
- [ ] Write up the GGH trapdoor construction
    - [ ] Write up the encryption scheme
    - [ ] Write up the signature scheme
- [ ] Where time permits, read the details of the LL algorithm and write about it in section 2

## References
- Application of Lattice trapdoor
    - [(Peikert, Waters, 08')Application of lossy trapdoor functions](https://eprint.iacr.org/2007/279.pdf)
    - [(Peikert, 09') Public-Key Cryptosystems from the Worst-Case Shortest Vector Problem](https://eprint.iacr.org/2008/481.pdf) (another CCA-secure system, but based on injective trapdoors)
- Lattice trapdoor construction
    - [(GGH, 97') Public-key cryptosystems from lattice reduction problems](https://www.wisdom.weizmann.ac.il/~oded/PSX/pkcs.pdf) generates lattice trapdoor by applying unimodular transformation to short basis to obtain bad basis
    - [(Ajtai, 99') Generating hard instances of the short basis problem](https://people.csail.mit.edu/vinodv/CS294/ajtai99.pdf)
    - [(Micciancio, 01')](https://cseweb.ucsd.edu/~daniele/papers/HNFcrypt.pdf) generate bad basis using Hermite normal form
    - [(Gentry, Peikert, Vaikuntanathan, 08') Trapdoors for hard lattices and new cryptographic constructions](https://eprint.iacr.org/2007/432)
    - [(Alwen, Peiker, 09') Generating shorter bases for hard random lattices](https://eprint.iacr.org/2008/521.pdf)
    - [(Micciancio, Peikert, 12') Trapdoors for lattices: simpler, tighter, faster, smaller](https://eprint.iacr.org/2011/501)
    - [(Micciancio, Genise, 17') Faster Gaussian sampling for trapdoor lattices with arbitrary modulus](https://eprint.iacr.org/2017/308)
    - [MIT CS294 lecture notes](https://people.csail.mit.edu/vinodv/CS294/lecturenotes.pdf)
- Digital signatures
    - [Practical Lattice-based Digital Signature Schemes](https://csrc.nist.gov/csrc/media/events/workshop-on-cybersecurity-in-a-post-quantum-world/documents/papers/session9-oneill-paper.pdf)
    - [(GLP 12') Practical Lattice-Based Cryptography: A Signature Scheme for Embedded Systems](https://www.iacr.org/archive/ches2012/74280529/74280529.pdf)