# A survey of lattice trapdoor generation
For Fall 23' CO 687's project I plan to survey the problem of lattice trapdoor generation. This topic is a natural extension from lecture materials covering LWE constructions in which both the lattice matrix $A$ and the secret $\vec{s}$ are uniformly randomly sampled from $\mathbb{Z}_q$, which have only limited potential for application compared to constructions that deploy a trapdoor, which typically takes on the form of a second lattice $S$ that is a subspace of the kernel $\ker(A)$ and is useful for solving SIS/LWE problems efficiently.

Theoretical construction of lattice trapdoor has been found since as early as 1999 by Ajtai, but early trapdoor generation schemes are complex and impractical due to high asymptotic complexity. Overtime, a number of improvements brought the problem of trapdoor generation into practical realms, culminating in Micciancio and Peiker's construction in 2012 for smooth modulus $q = 2^k$, later improved by Micciancio and Genise in 2017 to work on arbitrary modulus. This seems to be the state of the art for trapdoor generations, as I could not find newer works that provide further substantial improvements upon MP12's constructions.

Within the reasonable scope of 6-8 weeks, I plan to focus on providing a digest of a number of papers including AJ99, GPV08, AP09, MP12, and MG17, in each case discussing the trapdoor construction and their performance/security characteristics. If capacity permits, I would also like to cover some digital signature schemes based on lattice trapdoors, including their constructions and security arguments. Finally with further capacity permits, I would like to try to implement a trapdoor generation algorithm in Rust, as there is very little implementation on the public space at this moment.

General structure of each topic:

1. High level verview of the scheme
2. A formal statement
3. Correctness of inversion with the trapdoor
4. Hardness of inversion without the trapdoor
5. Applications

**Table of content**
- Preliminaries
    - [ ] Dual lattice is the lattice spanned by rows of $B^{-1}$
    - [ ] Two basis matrices describe the same lattice if and only if they are related by a unimodular matrix
    - [ ] [LLL basis reduction](https://math.mit.edu/~apost/courses/18.204-2016/18.204_Xinyue_Deng_final_paper.pdf)?
- Generate lattices with short basis from SIS (Ajtai '96, Ajtai '99)
    - [ ] Need to read Ajtai '96
    - [ ] Need more details on Ajtai '99
- Generate lattices with short basis, then use linear transformation to produce bad basis (GGH '97)
    - This paper is fairly readable
    - Additional material on [LLL](https://math.mit.edu/~apost/courses/18.204-2016/18.204_Xinyue_Deng_final_paper.pdf). LLL runs in polynomial time, but only provides exponentially bounded estimation of SVP; this means that when the number of dimension is large enough, LLL will not be able to provide meaningful recovery of the "good basis" from the "bad basis"


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