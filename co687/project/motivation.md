# Why trapdoor
We begin by recalling an early construction of a public-key cryptosystem based on the learning with errors (LWE) problem (Regev '05).

First, generate the parameters $(n, q, \chi, m) \leftarrow \text{PGen}(1^\lambda)$, where $n$ is the dimension of the LWE lattice, $q$ is the ambient modulus, $\chi$ is the error distribution, and $m$ is the number of LWE samples. Here $n$ is the main security parameter, where increasing $n$ makes the corresponding LWE problem harder to solve

During key generation, first uniformly sample the secret vector $\vec{s} \leftarrow \mathbb{Z}_q^n$, a public matrix $A \leftarrow \mathbb{Z}_q^{m\times n}$, and an error vector $\vec{e} \leftarrow \chi^m$. Compute the LWE observations $\vec{b} \leftarrow A\vec{s} + \vec{e} \in \mathbb{Z}_q^m$. The key pair is as follows:

$$
\begin{aligned}
\text{sk} &= \vec{s} \\
\text{pk} &= (A, \vec{b})
\end{aligned}
$$

To encrypt a message $m \in \{0, 1\}$, randomly sample a non-empty subset of the noisy linear equations and add them together, then add $m \cdot \lfloor \frac{q}{2} \rceil$ to the observation term.

$$
\begin{aligned}
\vec{x} &\leftarrow \{0, 1\}^m \\
\vec{c_1} &\leftarrow A^\intercal \vec{x} \in \mathbb{Z}_q^n\\
c_2 &\leftarrow \vec{b}^\intercal \vec{x} + m \cdot \lfloor \frac{q}{2} \rceil \in \mathbb{Z}_q \\
c &\leftarrow (\vec{c_1}, c_2)
\end{aligned}
$$

To decrypt the ciphertext, compute $c_2 - \langle \vec{c_1}, \vec{s} \rangle$:

$$
\begin{aligned}
c_2 - \langle \vec{c_1}, \vec{s} \rangle &= \vec{b}^\intercal \vec{x} + m \cdot \lfloor \frac{q}{2} \rceil - \langle \vec{c_1}, \vec{s} \rangle \\
&= \vec{b}^\intercal \vec{x} + m \cdot \lfloor \frac{q}{2} \rceil - \langle A^\intercal \cdot \vec{x}, \vec{s} \rangle \\
&= \vec{x}^\intercal \vec{b} + m \cdot \lfloor \frac{q}{2} \rceil - \vec{x}^\intercal A \vec{s}\\
&= \langle \vec{x}, \vec{b} - A\vec{s} \rangle + m \cdot \lfloor \frac{q}{2} \rceil \\
&= \langle \vec{x}, \vec{e} \rangle + m \cdot \lfloor \frac{q}{2} \rceil \\
&\approx m \cdot \lfloor \frac{q}{2} \rceil
\end{aligned}
$$

Then we check the result: if the result is closer to $\lfloor \frac{q}{2} \rceil$ then the decryption outputs 1, otherwise, decryption outputs 0.

In the original paper, Regev proved that the system above is IND-CPA secure assuming the hardness of the LWE problem. **However, this construction is not CCA-secure** because a CCA-adversary can sample an additional error term from the error distribution $\chi$, add it to $c_2$, then query the decryption oracle. Beacuse the modified ciphertext is distinct from the challenge ciphertext, the decryption oracle will perform the decryption, but the small error will be rounded out during the decryption process, so the adversary will be able to use the decryption oracle to recover the challenge plaintext.

As was pointed out by Micciancio and Peikert in 2012 (MP'12), LWE problems constructed using uniformly sampled matrices and secrets generally only provide passive security (IND-CPA). In addition, Peikert and Waters (PW'08) provided a block-box construction of a CCA-secure public-key cryptosystem using lossy trapdoors. For the remainder the project, we will survey algorithms for generating lattice trapdoors and their applications in building CCA-secure cryptosystem and digital signatures among many others.

## References
- [(Regev '05) On Lattices, Learning with Errors, Random Linear Codes, and Cryptography](https://cims.nyu.edu/~regev/papers/qcrypto.pdf)
- [(Micciancio, Peikert, 12') Trapdoors for lattices: simpler, tighter, faster, smaller](https://eprint.iacr.org/2011/501)
- [(Peikert, Waters, 08') Application of lossy trapdoor functions](https://eprint.iacr.org/2007/279.pdf)