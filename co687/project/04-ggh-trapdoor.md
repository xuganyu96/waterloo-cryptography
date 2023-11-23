# The Goldreich-Goldwasser-Halevi trapdoor
One class of constructing lattice trapdoor uses a pair of public and secret basis for the same lattice. Since the two basis generate the same lattice, they are equally good at mapping integer coordinates into a lattice point. On the other hand, the secret basis is very "good" and can be used to efficiently find the closest lattice point, but the public basis is very "bad" at recovering the closest lattice point.

One of the earliest instances of such class of lattice trapdoor is proposed by Goldreich, GoldWasser, and Halevi in their 1997 paper *"Public-key cryptosystem from lattice reduction problem"*. At a high level, the trapdoor is parameterized by three items: a "bad" basis $B$, a "good" basis $R$, and an error bound $\sigma$. In the forward direction, the function maps a pair of lattice coordinate $\mathbf{v} \in \mathbb{Z}^n$ and a small error vector $\mathbf{e} \leftarrow \{ -\sigma, \sigma \}^n$ to $\mathbf{x} = B\mathbf{v} + \mathbf{e} \in \mathbb{R}^n$. If the parameters are generated correctly, then the closest lattice point in $\mathcal{L}(B)$ is exactly $B\mathbf{v}$.

Inverting the function involves recovering the integer coordinate $\mathbf{v}$ (or the error vector $\mathbf{e}$, since recovering one of them automatically gives you the other). However, the inversion is exactly the cloest vector problem (CVP), and should be hard if $B$ is a sufficiently bad basis. On the other hand, since $R$ is a good basis, finding the closest vector point should be "easy" if we have $R$.

From here, GGH '97 proposed a public-key cryptosystem as well as a digital signature scheme that uses such a trapdoor construction, and the security of the two schemes naturally rest on the hardness of the underlying hard lattice problem.

## The trapdoor scheme
The GGH trapdoor scheme contains four components:

1. Parameter generation  
The main security parameter in this scheme is the number of dimensions $n$ of the lattice. In the original paper, the authors claimed $n \approx 150$ is sufficient for making inverting the function without the private basis hard and $n \approx 250$ should be a safe choice for the foreseeable future.
2. Key generation  
First generate the "good" basis $R \in \mathbb{R}^{n \times }$, then apply some (unimodular) transformation to $R$ to obtain the "bad" basis $B$. The error bound $\sigma > 0 \in \mathbb{R}$ is dependent on the choice of $R$ and the choice of "probability of inversion error" $\epsilon >= 0$, which will be discussed in a later section.
3. Forward evaluation  
$f_{B}(\mathbf{v}, \mathbf{e}) = B\mathbf{v} + \mathbf{e}$, where $\mathbf{v} \in \{-n, \ldots, n\}^n$ and $\mathbf{e} \in \{-\sigma, \sigma\}$. According to the authors, the choice of bounds for values of $\mathbf{v}$ is arbitrary and not a significant contributor to the overall security of the scheme.
4. Inversion  
Denote the output by $\mathbf{x} = f_{B}(\mathbf{v}, \mathbf{e})$, first attempt to recover the integer coordinate $\mathbf{v} \leftarrow B^{-1}R\lfloor R^{-1}\mathbf{x} \rceil$. From here it is trivial to recompute the lattice point $B\mathbf{v}$ and recover the error term $\mathbf{e} = \mathbf{x} - B\mathbf{v}$.

### Correctness of trapdoor inversion
Without the error term, the function $f_B: \mathbf{v} \mapsto B\mathbf{v}$ is trivially invertible with either choice of the basis. However, with a non-zero the error term, the quality of the basis makes a substantial difference in how much error can be added before the points can no longer be recovered.

Observe the calculation used for recovering the integer coordinate $\mathbf{v}$:

$$
\begin{aligned}
\lfloor R^{-1}\mathbf{x} \rceil &= B^{-1}R\lfloor R^{-1}(B\mathbf{v} + \mathbf{e})\rceil \\
&= B^{-1}R\lfloor R^{-1}B\mathbf{v} + R^{-1}\mathbf{e}\rceil
\end{aligned}
$$

Since $R, B$ are related by a unimodular matrix and $\mathbf{v}$ is an integer vector, $R^{-1}B\mathbf{v}$ is an integer vector and can be moved out of the "rounding" operator:

$$
\begin{aligned}
\lfloor R^{-1}\mathbf{x} \rceil &= B^{-1}RR^{-1}B\mathbf{v} + B^{-1}R\lfloor R^{-1}\mathbf{e}\rceil \\
&= \mathbf{v} + B^{-1}R\lfloor R^{-1}\mathbf{e} \rceil
\end{aligned}
$$

Since $B^{-1}R$ is also a unimodular matrix, we can conclude that the equation above is successful at recovering the original coordinate if and only if $R^{-1}\mathbf{e} = \mathbf{0}$.

To guarantee that inversion error never happens, we can bound the error term $\sigma > 0$ by $\frac{1}{2\rho}$, where $\rho$ is maximal $L_1$ norm among the rows of $R^{-1}$. This bound is excessively conservative, however, and we might want to relax the bound to enhance the security of the trapdoor scheme (larger error terms makes it harder to invert the function using only the public basis). The authors provided one such relaxation based on the Hoeffding inequality. This relaxation is stated as follows

$$
P(\text{inversion error}) \leq 2n \cdot \exp(-\frac{1}{8\sigma^2\gamma^2})
$$

Where $\gamma = \sqrt{n} \cdot \max(L_\infty \text{ norm of rows of } R^{-1})$. Simple reorganization of inequality shows that to bound the inversion error by $\epsilon$, the error term will be bounded by $\sigma \leq (\gamma\sqrt{8\ln{2n/\epsilon}})^{-1}$.

### Generating the pair of basis
The authors described two similar ways of generating the private basis. The first way is to sample each coordinate of $R \in \mathbb{R}^{n \times n}$ from a uniform distribution on $\{-l, -l + 1, \ldots, l-1, l\}$, where according to the authors, the value of the bound $l$ has negligible impact on the quality of the generated basis (the authors chose $\pm 4$ in their implementation). A second method is to first generate a square lattice $L(kI)$ for some positive integer $k$, then add a small amount of noise $R^\prime \in \{-l, \ldots, l\}^{n \times n}$. With this method of sampling the private basis, it is important to balance the choice of values between $k$ and $l$, where a larger $k$ value gives a more orthogonal basis, but also weakens the security of the trapdoor function by making it easier to reduce the public basis into a short, orthogonal basis using basis reduction algorithm.

The authors also described two methods for generating the public basis $B$ from the private basis. The first method is to directly generate random unimodular matrix $T$ and set $B = TR$, then repeat until satisfactory. A second method is to repeatedly apply column mixing, where at each mixing a column of $R$ is chosen, and a linear combination of all other columns is added to the chosen column.

Mathematically, the two methods are equivalent. However, in implemenetation, we would like the values of $B$ to be smaller for space efficiency while maintaining sufficient security so that the function cannot be easily inverted using the public basis alone and so that $B$ cannot be easily reduced using basis reduction algorithm. The authors preferred column mixing for requiring less computation and for producing public basis $B$ with smaller values.

Unfortunately, there is no known rigorous description of how skewed $B$ needs to be for the trapdoor function to be secure. The authors relied on experimental methods and determined that for $n \approx 100$, $2n$ steps of column mixing is enough to render LLL basis reduction ineffective at meaningfully improving the quality of the public basis $B$.

### Inverting the function without the trapdoor
From a high level, inverting the function corresponds to finding the an approximate closest lattice point, so the best attacks on the trapdoor are also algorithms for solving the CVP problem.

The most straightforward attack on the trapdoor is to simply run the inversion algorithm with the public basis instead of the private basis:

$$
B^{-1}\mathbf{x} = B^{-1}(B\mathbf{v} + \mathbf{e}) = \mathbf{v} + B^{-1}\mathbf{e}
$$

Due to $B$ being very skewed, this procedure does not immediately yield the correct value for $\mathbf{v}$ since $B^{-1}\mathbf{e}$  is not $\mathbf{0}$. However, since the possible values of $\mathbf{e}$ is finite, we can perform an exhaustive search on all possible values of $\mathbf{d} = B^{-1}\mathbf{e}$, although the search space will grow exponentially with the number of dimensions $n$. In experimental settings, with $n \geq 100$ the search space reaches 168 bits of entropy.

Using a better approximation algorithm for CVP, such as the nearest plane algorithm, yields a more efficient inversion than the brute-force search described above, although the complexity of the algorithm still scales exponentially with $n$. In experimental settings, with $n=150$ the workload of the nearest plane algorithm reaches 104 bits.

### Improvement using Hermite normal form
In 2001, Daniel Micciancio proposed an improvement to the GGH trapdoor function by suggesting that the public basis $B$ be generated as the Hermite normal form of the private basis $R$. The main improvement comes from two characteristics of 

1. Hermite normal form is unique for a lattice, so now there is a strong guarantee that the public basis gives no information about the private basis, whereas previously GGH did not provide any formal proof of security on this front and relied on experimental results
2. Hermite normal form is triangular, which means that encoding the public basis takes less space

In the 2001 paper, Micciancio proved that the HNF trapdoor function is at least as secure as the original GGH trapdoor.

## Applications
The GGH paper presented two possible applications of a GGH-like trapdoor function: a public-key cryptosystem and a digital signature.

- What is the cryptosystem
- What is the digital signature

Unfortunately, in 1999 the public-key cryptosystem was cryptanalyzed by Nguyen, and in 2006 the digital signature scheme was also broken by Nguyen and Regev. Since then, newer lattice trapdoors based on gadget matrix have been proposed, so we will not cover details from the cryptanalysis in this survey.


## References
- [(Nguyen '97) Cryptanalysis of the Goldreich–Goldwasser–Halevi Cryptosystem](https://link.springer.com/content/pdf/10.1007/3-540-48405-1_18.pdf)
- [(Nguyen, Regev, '06) Learning a Parallelepiped: Cryptanalysis of GGH and NTRU Signatures](https://iacr.org/archive/eurocrypt2006/40040273/40040273.pdf)