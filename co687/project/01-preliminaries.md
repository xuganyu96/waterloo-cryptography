Topics to cover in the preliminaries

- [Lattices](#lattices)
    - [The dual lattice](#the-dual-lattice)
- [Hard lattice problems]()
    - [Closest vector problem](#closest-vector-problem)

# Lattices

## The dual lattice
https://cseweb.ucsd.edu/classes/wi12/cse206A-a/LecDual.pdf

Let $B \in \mathbb{R}^{n \times n}$ define a lattice $\mathcal{L}(B)$. The dual lattice is defined by:

$$
\mathcal{L}^\wedge = \{\bold{b} \in \text{span}(\mathcal{L}) \mid \langle\bold{b}, \bold{v}\rangle \in \mathbb{Z}, \forall \bold{v} \in \mathcal{L} \}
$$

Where $\langle \cdot, \cdot \rangle$ is the real inner product.

* The dual lattice of $B$ has a basis of $B^{-1}$ where the inversion takes place in the real number

# Hard lattice problems

## Closest vector problem
In a general setting, the closest vector problem asks to find the lattice point $\bold{v} \in \mathcal{L}$ that is closest to some given arbitrary point $\bold{x} \in \mathbb{R}^n$.

- Solving the exact CVP problem is NP hard
- Finding an approximate closest vector $\bold{v} \in \mathcal{L}$ such that $\Vert \bold{v} - \bold{x} \Vert \leq \lambda$ for some constant $\lambda$ is NP hard
- Babai and Schnorr each provide a polynomial-time algorithm that can output an approximation that is within an exponential factor ($\lambda \in O(2^\frac{n}{2})$ for Babai, $\lambda \in O((1+\epsilon)^n)$ for arbitrary $\epsilon$ for Schnorr)

## Finding a small basis
Let $\mathcal{L}$ be a lattice (defined by some basis $B \in \mathbb{R}^{n \times n}$), find a basis for $\mathcal{L}$ that is "small"

- One definition of "small" can be "to have small orthogonality-defect"