# Lattice trapdoor using reduced basis
One class of constructing lattice trapdoor uses a pair of public and secret basis for the same lattice. Since the two basis generate the same lattice, they are equally good at mapping integer coordinates into a lattice point. On the other hand, the secret basis is very "good" and can be used to efficiently find the closest lattice point, but the public basis is very "bad" at recovering the closest lattice point.

One of the earliest instances of such class of lattice trapdoor is proposed by Goldreich, GoldWasser, and Halevi in their 1997 paper *"Public-key cryptosystem from lattice reduction problem"*. At a high level, the trapdoor is parameterized by three items: a "bad" basis $B$, a "good" basis $R$, and an error bound $\sigma$. In the forward direction, the function maps a pair of lattice coordinate $\bold{v} \in \mathbb{Z}^n$ and a small error vector $\bold{e} \leftarrow \{ -\sigma, \sigma \}^n$ to $\bold{x} = B\bold{v} + \bold{e} \in \mathbb{R}^n$. If the parameters are generated correctly, then the closest lattice point in $\mathcal{L}(B)$ is exactly $B\bold{v}$.

Inverting the function involves recovering the integer coordinate $\bold{v}$ (or the error vector $\bold{e}$, since recovering one of them automatically gives you the other). However, the inversion is exactly the cloest vector problem (CVP), and should be hard if $B$ is a sufficiently bad basis. On the other hand, since $R$ is a good basis, finding the closest vector point should be "easy" if we have $R$.

From here, GGH '97 proposed a public-key cryptosystem as well as a digital signature scheme that uses such a trapdoor construction, and the security of the two schemes naturally rest on the hardness of the underlying hard lattice problem.

In the remainder of the section we will formally state the trapdoor scheme, discuss correctness, security, and performance. We will then briefly discuss subsequent improvements by Micciancio using HNF.

## The trapdoor scheme
The GGH trapdoor scheme contains four components:

1. Parameter generation  
Generate public and secret basis $B, R \in \mathbb{R}^{n \times n}$ and a positive real number $\sigma > 0$. The public basis $B$ and the error term $\sigma$ constitute the description of the evaluation function, and the private basis $R$ is the trapdoor.
2. Sampling inputs  
Uniformly sample $\bold{v} \leftarrow \{-n, -n+1, \ldots, n-1, n\}$ and $\bold{e} \leftarrow \{-\sigma, \sigma\}$
3. Forward evaluation  
$f_{B}(\bold{v}, \bold{e}) = B\bold{v} + \bold{e}$
4. Inversion  
Denote the output by $\bold{x} = f_{B}(\bold{v}, \bold{e})$...

### Correctness of inversion
We need to bound the 

