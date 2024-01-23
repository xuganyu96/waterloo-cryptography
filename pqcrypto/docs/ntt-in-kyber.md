# NTT in Kyber
NTT in Kyber is most intuitively interpreted using the Chinese Remainder Theorem (CRT). The parameters of Kyber are chosen such that $q = 3329 = 2^8 \cdot 13 + 1$, which means that there exists a 256-th root of unity $\zeta$ in $\mathbb{F}_q$ but not a 512-th root of unity. Because $\zeta^{256} \equiv 1$ but $\zeta^{128}$ as a square root of $\zeta^{256}$ must not be 1 (because 256-root of unity), it must be that $\zeta^{128} \equiv -1$.

Within $\mathbb{Z}_q[x]$ we can factor the quotient polynomial $x^{256} + 1$ as follows:

$$
\begin{aligned}
x^{256} + 1 &= x^{256} - \zeta^{128} \\
&= (x^{128} - \zeta^{64})(x^{128} + \zeta^{64}) \\
&= (x^{128} - \zeta^{64})(x^{128} - \zeta^{128} \cdot \zeta^{64}) \\
&= (x^{64} - \zeta^{32})(x^{64} + \zeta^{32})
    (x^{64} - \zeta^{96})(x^{64} + \zeta^{96}) \\
&\ldots \\
&= (x^2 - \zeta)(x^2 - \zeta^3)\ldots(x^2 - \zeta^{255})
\end{aligned}
$$

Odd powers of $\zeta$ could not have square roots, so each factor at the RHS is irreducible.

From the (generalized) CRT we know that

$$
\mathbb{Z}_q[x] / \langle
    x^{256} + 1
\rangle
\cong 
\mathbb{Z}_q[x] / \langle
    x^2 - \zeta
\rangle
\times \mathbb{Z}_q[x] / \langle
    x^2 - \zeta^3
\rangle
\times \ldots \times \mathbb{Z}_q[x] / \langle
    x^2 - \zeta^{255}
\rangle
$$

So we can map a polynomial in $R_q = \mathbb{Z}_q[x] / \langle x^{256} + 1 \rangle$ into the NTT space on the R.H.S. and perform arithmetic in the NTT space (in fact, there is no reason why we need representation using the L.H.S. except for readability; all polynomials within Kyber will stay in the NTT representation as the default)

To map a polynomial from $R_q$ to NTT domain we need to compute polynomial modulus reduction. Fortunately, the reduction is straightforward because $x^2 - \zeta \equiv 0 \mod (x^2 - \zeta)$ means $x^2 \equiv \zeta \mod (x^2 - \zeta)$. Therefore:

$$
\begin{aligned}
f_0 + f_1x + \ldots + f_{255}x^{255} 
&\equiv f_0 + f_1x 
    + f_2\zeta + f_3\zeta x 
    + \ldots 
    + f_{254}\zeta^{127} + f_{255}\zeta^{127}x 
    \mod (x^2 - \zeta) \\
&\equiv (f_0 + f_2\zeta + \ldots + f_{254}\zeta^{127})
    + (f_1 + f_3\zeta + \ldots + f_{255}\zeta^{127})x
    \mod (x^2 - \zeta)
\end{aligned}
$$

This computation generalizes to the formula presented in the specification:

$$
\begin{aligned}
f_{2i} &= \sum_{j=0}^{127}f_{2j}(\zeta^{2i+1})^j \\
f_{2i + 1} &= \sum_{j=0}^{127}f_{2j + 1}(\zeta^{2i+1})^j
\end{aligned}
$$

## In-place transformation
The canonical representation of $f \in R$ contains 256 coefficients. The NTT domain consists of 128 polynomials of degree 1, where each "polynomial residue" is canonically represented using its two coefficients. Therefore, the NTT domain can also be represented using 256 coefficents, making the NTT transformation suitable for an in-place transformation. In other words, given an input of an array of 256 integers, the transformation will simply mutate the array instead of creating new arrays.