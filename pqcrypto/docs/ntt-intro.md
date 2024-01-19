# Introduction to number theoretic transform (NTT)
With Ring-LWE based cryptography scheme, a significant performance bottleneck is the multiplication of polynomial in a quotient ring. Here we discuss some naive multiplication algorithm, then introduce the number theoretic transform in some restricted cases and prove some preliminary results.

## School-book multiplication
We are primarily concerned with two categories of polynomial rings:

1. $\mathbb{Z}_q[x] / \langle x^n - 1 \rangle$ where $q$ is prime, $n$ is a power of 2, and $n \mid (q-1)$
2. $\mathbb{Z}_q[x] / \langle x^n + 1 \rangle$ where $q$ is prime, $n$ is a power of 2, and $2n \mid (q-1)$

We first consider performing multiplication in the first ring $R = \mathbb{Z}_q[x] / \langle x^n - 1 \rangle$. Let $\mathbf{a} \in R$ be some arbitrary polynomial in this quotient ring:

$$
\mathbf{a} = a_0 + a_1x + \ldots + a_{n-1}x^{n-1}
$$

Then

$$
\begin{aligned}
\mathbf{a} \cdot x
&= a_0x + a_1x^2 + \ldots + a_{n-1}x^n \\
&\equiv a_0x + a_1x^2 + \ldots + a_{n-1}x^n  - a_{n-1}(x^n - 1) \mod (x^n-1) \\
&\equiv a_{n-1} + a_0x + \ldots + a_{n-2}x^{n-1}
\end{aligned}
$$

If we lay out the coefficients of $\mathbf{a}$ and $\mathbf{a}\cdot x$ into vectors, we find that

$$
(a_0, a_1, \ldots, a_{n-1}) \cdot x = (a_{n-1}, a_0, a_1, \ldots, a_{n-2})
$$

In other words, the coefficients are cyclically right shifted by one position (for that reason we call multiplication in this ring "cyclic convolution").

From here it is easy to see that multiplying by $x^k$ is equivalent to cyclic right shifting by $k$ positions (for $0 \leq k < n$). We also know that addition and scalar multiplication are performed like they are done in a regular vector space. Thus, we have the schoolbook multiplication formula for polynomials in this ring

> Let $\mathbf{a} = (a_0, a_1, \ldots, a_{n-1}), \mathbf{b} = (b_0, b_1, \ldots, b_{n-1})$ be coefficients of two polynomials in the ring $R = \mathbb{Z}_q[x] / \langle x^n - 1 \rangle$, then $\mathbf{a} \cdot \mathbf{b} = (c_0, c_1, \ldots, c_{n-1})$, where $c_i = \sum_{j=0}^{i}a_{i-j}b_j + \sum_{j=i+1}^{n-1}a_{n+i-j}b_j$

Schoolbook multiplication in the other ring $\mathbb{Z}_q[x] / \langle x^n + 1 \rangle$ follows a similar argument, although the the shifting behavior is slightly different:

$$
\begin{aligned}
(a_0 + a_1x + \ldots + a_{n-1}x^{n-1}) \cdot x
&= a_0x + a_1x^2 + \ldots + a_{n-1}x^n \\
&\equiv a_0x + a_1x^2 + \ldots + a_{n-1}x^n  - a_{n-1}(x^n + 1) \\
&\equiv -a_{n-1} + a_0x + a_1x^2 + \ldots + a_{n-2}x^{n-1} 
\end{aligned}
$$

In other words, in addition to a cyclic right shift, the wrapped elements are also negative (hence "negative-wrapping convolution"): $(a_0, a_1, \ldots, a_{n-1}) \cdot x = (-a_{n-1}, a_0, a_1, \ldots, a_{n-2})$. The computation of $\mathbf{a} \cdot \mathbf{b}$ is thus almost identical to the multiplication formula in cyclic convolution, except for that the wrapped elements are flipped.

> In $R = \mathbb{Z}_q[x] / \langle x^n + 1 \rangle$, $c_i = \sum_{j=0}^{i}a_{i-j}b_j - \sum_{j=i+1}^{n-1}a_{n+i-j}b_j$

## Cyclic NTT
Let's consider the polynomial ring $R = \mathbb{Z}_q / \langle x^n - 1 \rangle$.

If $q$ is prime, then there exists a primitive root $\omega \in \mathbb{Z}_q^\ast$ of the multiplicative group. In addition, if $n \mid q-1$, then there exists an n-th primitive root, and we can compute it:

$$
\omega_n \equiv \omega^\frac{q-1}{n} \mod q
$$

The forward NTT transformation maps a polynomial in the coefficient space $(a_0, a_1, \ldots, a_{n-1})$ into the NTT space $\text{NTT}(\mathbf{a}) = (\hat{a}_0, \hat{a}_1, \ldots, \hat{a}_{n-1})$, where

$$
\hat{a}_j = \sum_{i=0}^{n-1} a_i(\omega_n^j)^i
$$

The inverse transformation, which we denote by $\text{INTT}(\hat{\mathbf{a}}) = (a_0, a_1, \ldots, a_{n-1})$ is as follows:

$$
a_i = n^{-1} \sum_{j=0}^{n-1}\hat{a}_j(\omega_n^{-i})^j
$$

where $n^{-1}$ is defined modulus inversion under $q$.

Here we state the correctness of the inversion:

> $$ \text{INTT}(\text{NTT}(\mathbf{a})) = \mathbf{a} $$

To prove the correctness of the inversion, notice that both NTT and INTT are matrix multiplication: $\text{NTT}(\mathbf{a}) = W\mathbf{a}$ where

$$
W = \begin{bmatrix}
\omega_n^{0 \cdot 0} & \omega_n^{0 \cdot 1} & \ldots & \omega_n^{0 \cdot (n-1)} \\
\omega_n^{1 \cdot 0} & \omega_n^{1 \cdot 1} & \ldots & \omega_n^{1 \cdot (n-1)} \\
&&\ldots \\
\omega_n^{(n-1) \cdot 0} & \omega_n^{(n-1) \cdot 1} & \ldots & \omega_n^{(n-1) \cdot (n-1)} \\
\end{bmatrix}
$$

On the other hand $\text{INTT}(\hat{\mathbf{a}}) = n^{-1}Y\hat{\mathbf{a}}$, where

$$
Y = \begin{bmatrix}
\omega_n^{-0 \cdot 0} & \omega_n^{-0 \cdot 1} & \ldots & \omega_n^{-0 \cdot (n-1)} \\
\omega_n^{-1 \cdot 0} & \omega_n^{-1 \cdot 1} & \ldots & \omega_n^{-1 \cdot (n-1)} \\
&&\ldots \\
\omega_n^{-(n-1) \cdot 0} & \omega_n^{-(n-1) \cdot 1} & \ldots & \omega_n^{-(n-1) \cdot (n-1)} \\
\end{bmatrix}
$$

Thus $\text{INTT}(\text{NTT}(\mathbf{a})) = n^{-1}YW\mathbf{a}$.

The i-th row of $Y$ contains $(\omega_n^{-i \cdot 0}, \omega_n^{-i \cdot 1}, \ldots, \omega_n^{-i \cdot (n-1)})$. The j-th column of $W$ contains $(\omega_n^{0 \cdot j}, \omega_n^{1 \cdot j}, \ldots, \omega_n^{(n-1) \cdot j})$. Therefore:

$$
(YW)_{i,j} = \sum_{k=0}^{n-1}\omega_n^{-ik}\omega_n^{jk} = \sum_{k=0}^{n-1}(\omega_n^{i-j})^k
$$

Where $i=j$, the right hand side can be easily computed: $\sum_{k=0}^{n-1}(\omega_n^{i-j})^k = \sum_{k=0}^{n-1}(\omega_n^0)^k = n$.

Where $i\neq j$, notice the right hand side is a geometric series:

$$
\sum_{k=0}^{n-1}(\omega_n^{i-j})^k = \frac{
    (\omega_n^{i-j})^n - (\omega_n^{i-j})^0
}{\omega_n^{i-j} - 1}
$$

Notice in the numerator term $(\omega_n^{i-j})^n = (\omega_n^n)^{i-j} = 1$ since $\omega_n$ is the n-th primitive root. In the denominator term, $\omega_n^{i-j} \neq 1$ since $0 \leq i, j < n$, meaning $-(n-1) \leq i - j \leq n-1$. Therefore, the whole fraction evaluates to $0$.

In other words, the diagonal entries of $YW$ are all $n$ while all other entries are $0$, meaning $YW = nI$. Therefore $\text{INTT}(\text{NTT}(\mathbf{a})) = n^{-1}YW\mathbf{a} = n^{-1}nI\mathbf{a} = \mathbf{a}$

## Negative-wrapped NTT
Let $R = \mathbb{Z}_q[x] / \langle x^n + 1 \rangle$ where $q$ is a prime number, $n$ is a power of 2, and $2n \mid q-1$. Using similar argument as above, we know that there exist a 2n primitive root $\phi$ as well as an n-th primitive root $\omega$.

The NTT transformation and inversion in negative-wrapped convolution follows similar structure but with some additional terms:

$$
\begin{aligned}
\text{NTT}^\phi(\mathbf{a}) &= \text{NTT}(\langle \Phi, \mathbf{a} \rangle) \\
\text{INTT}^\phi(\hat{\mathbf{a}}) &= \langle \Phi^{-1}, \text{INTT}(\hat{\mathbf{a}}) \rangle
\end{aligned}
$$

Where $\Phi = (\phi^0, \phi^1, \ldots, \phi^{n-1})$ and $\Phi^{-1} = (\phi^0, \phi^{-1}, \ldots, \phi^{-(n-1)})$.