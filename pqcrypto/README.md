# Software implementation of CRYSTALS-Dilithium

# Cyclic convolution

Given prime $q$, and a power of two $n = 2^k$, define the polynomial ring:

$$
R \leftarrow \mathbb{Z}_q[x] / \langle x^n - 1 \rangle
$$

A polynomial $\mathbf{a}$ in this polynomial ring can be represented using a set of $n$ coefficients:

$$
\mathbf{a} = \sum_{i=0}^{n-1}a_ix^i
$$

Where $n \mid q-1$, there exists an n-th primitive root $\omega_n$ in the multiplicative group $\mathbb{Z}_q^\ast$ (how to find it?). Using $\omega_n$ we can define the forward number theoretic transformation of the polynomial $\mathbf{a}$ from the time domain to the frequency domain:

$$
\hat{\mathbf{a}} \leftarrow \mathop{\text{NTT}}(\mathbf{a})
$$

where for $0 \leq j < n$:

$$
\hat{a}_j = \sum_{i=0}^{n-1}a_i \omega_n^{ij}
$$

Note that coefficient addition and multiplication are defined within the integer ring $\mathbb{Z}_q$.

This formula is a bit daunting to work with in its raw form. Instead, notice that $\hat{a}_j$ is computed by evaluating the polynomial at some value:

$$
\begin{aligned}
\hat{a}_j &= \sum_{i=0}^{n-1}a_i\omega_n^{ij} \\
&= \sum_{i=0}^{n-1}a_i(\omega_n^j)^i
\end{aligned}
$$

In other words, we are evaluating the polynomial $a_0 + a_1x + \ldots + a_{n-1}x^{n-1} \in R$ at the $n$ distinct powers of the n-th primitive root $\omega_n$.

Collecting $n$ points is necessary and sufficient for determining the coefficients of a polynomial in $R$. This transformation is referred to as transforming the polynomial from its coefficient representation to its evaluation representation. Evaluation representation is particularly useful because when computing the product of two polynomials (in their evaluation form), the evaluation of the product at some $x$ is exactly the product of the evaluations of the two operand polynomial at the same $x$:

$$
f(x) = g(x)h(x) \rightarrow f(x_0) = g(x_0)h(x_0)
$$

This means that the evaluation representation can be used to compute products of polynomials in $O(n)$ time (compute the product of each pair of points) instead of $O(n^2)$ time. It remains to find a way to convert from evaluation representation back to coefficient representation.

The inverse transformation:

$$
a_j \leftarrow n^{-1}\sum_{i=0}^{n-1}\hat{a}_i\omega_n^{-ij}
$$

The frequency domain representation of a polynomial is useful because it makes polynomial multiplication in this quotient ring more efficient:

$$
\hat{\mathbf{c}} = \langle \hat{\mathbf{a}}, \hat{\mathbf{b}} \rangle
$$

Where the inner product is defined as coefficient-wise multiplication within $\mathbb{Z}_q$