# Ajtai's short basis generation

Recall the short integer solution $\operatorname{SIS}(n, m, q, \beta)$

$$
A\vec{x} = \vec{0}
$$

Where $A \leftarrow \mathbb{Z}_q^{n \times m}$ is uniformly randomly sampled, and we are looking for $x \in \mathbb{Z}^m$  satisfying the equation such that the "length" of $\vec{x}$ is bounded by the parameter $\beta$. The short integer solution problem defined a lattice $\mathcal{L}(A^\bot)$:

$$
\mathcal{L} = \mathcal{L}(A^\bot) = \{x \in \mathbb{Z}^m \mid Ax = 0\}
$$

Solving the $\operatorname{SIS}(n, m, q, \beta)$ is thus equivalent to solving the shortest vector problem in $\mathcal{L}$.

If $A$ is truly uniformly random, then finding short solution is hard (source?). However, Ajtai showed in AJTAI'96 that it is possible to generate an instance of an SIS problem with a known short solution while the generated $A^\prime$ has negligible statistical distance from truly random. In AJTAI'99, Ajtai improved the generation algorithm to produce a full basis $S$ for $\mathcal{L}$ such that $AS = 0$.

## Generate SIS with short vector
The algorithm for generating an SIS problem with a known short vector contains the following steps:

1. Independently sample $n - 1$ vectors $a_i \leftarrow \mathbb{Z}_q^n$
2. Independently sample $n - 1$ bits $b_i \leftarrow \{0, 1\}$
3. Compute $a_n = \sum_{i=1}^{n-1} a_ib_i$
4. $A = \lbrack a_1, a_2, \ldots, a_n \rbrack$ has known short solution $x = (b_1, b_2, \ldots, b_{n-1}, 1)$

Ajtai proved (AJTAI'96) that the generated instance $A$ has negligible statistical distance from uniformly random samples in $\mathbb{Z}_q^{n \times n}$