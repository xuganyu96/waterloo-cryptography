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
\hat{a_j} = \sum_{i=0}^{n-1}a_i \omega_n^{ij}
$$

Note that coefficient addition and multiplication are defined within the integer ring $\mathbb{Z}_q$.

The inverse transformation:

$$
a_j \leftarrow n^{-1}\sum_{i=0}^{n-1}\hat{a}_i\omega_n^{-ij}
$$

The frequency domain representation of a polynomial is useful because it makes polynomial multiplication in this quotient ring more efficient:

$$
\hat{\mathbf{c}} = \langle \hat{\mathbf{a}}, \hat{\mathbf{b}} \rangle
$$

Where the inner product is defined as coefficient-wise multiplication within $\mathbb{Z}_q$