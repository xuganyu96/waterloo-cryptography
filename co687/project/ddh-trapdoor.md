# A lossy trapdoor based on DDH
For an example of a lossy trapdoor, Peikert and Waters described a construction based on the Decisional Diffie-Hellman assumption.

- Generate the DDH parameters: a prime number $p$, a cyclic group $G$ of order $p$, and a random element frmo the group $g \leftarrow G$. 
- Independently sample $n$ ElGamal key pairs $(\text{sk} = z_i, \text{pk} = h_i = g^{z_i})$
- Independently sample $n$ random elements from $r_1, r_2 \ldots, r_n \leftarrow \mathbb{Z}_p$
- Use the $n$ key pairs and the $n$ random elements to encrypt either $M = I \in \mathbb{Z}_p^{n \times n}$ (for injective trapdoor) or $M = 0$ (for lossy trapdoor)

The encryption of a matrix consists of two elements $C_1, C_2$, where

$$
C_1 = \begin{bmatrix}
g^{r_1} \\ g^{r_2} \\ \ldots \\ g^{r_n}
\end{bmatrix}
$$

and 

$$
C_2[i, j] = g^{M[i,j]} (g^{z_j})^{r_i}
$$

The function index is the matrix encryption $C = (C_1, C_2)$, the trapdoor is the set of ElGamal secret keys $t = (z_1, z_2, \ldots, z_n)$.

To evaluate the trapdoor function on $x \leftarrow \{0, 1\}^n$, compute the following two things:

1. The inner product of $C_1$ and $x$, except multiplication is replaced by raising $C_1[i]$ to $x[i]$ for each $i$.
1. The inner product of each column of $C_2$ and $x$, except multiplication is replaced by "raising $C_2[i, j]$ to $x[i]$ exponent" for $1 \leq i \leq n$.

In other words, $F(C, x) = (y_1, y_2) \in \mathbb{Z}_p \times \mathbb{Z}_p^n$ where

$$
\begin{aligned}
y_1 &= g^{\sum_{i=1}^nx_ir_i} \\
y_2[j] &= g^{\sum_{i=1}^nx_im_{i,j}} \cdot (g^{z_j})^{\sum_{i=1}^{n}x_ir_i}
\end{aligned}
$$

Note that only when the trapdoor is injective, $m_{i, j} = 1$ for $i = j$, so we have $y_2[j] = g^{x_j} \cdot (g^{z_j})^{\sum_{i=1}^{n}x_ir_i}$. For when the trapdoor is lossy, $y_2[j] = (g^{z_j})^{\sum_{i=1}^{n}x_ir_i}$.

When the trapdoor is injective, we can recover the input $x$ bit by bit. For bit $j$, we use the secret key $z_j$ and the witness $y_1$ to decrypt $y_2[j]$.

When the trapdoor is lossy, **what is the lossiness?**

**How is injective trapdoor indistinguishable from lossy trapdoor?**


## References
- [(Peikert, Waters, 08') Application of lossy trapdoor functions](https://eprint.iacr.org/2007/279.pdf)