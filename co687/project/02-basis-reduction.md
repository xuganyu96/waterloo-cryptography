# Lattice basis reduction
In this section we discuss some known best bounds on the shortest vector (of a lattice), as well as the known best algorithm for approximating the shortest vector.

By convention, we define the minimum distance of a lattice to be the minimum of all distances between two distinct points on a lattice. Because lattice is a discrete sub-group of $\mathbb{Z}^n$, this minimum distance is always achieved by some pair of points. Also because a lattice is closed under addition (and thus subtraction), the minimum distance is exactly the length of the shortest vector on the lattice. Denote the mininum distance by $\lambda(\mathcal{L})$. For convenience, where there is no confusion of which lattice we are working with, we simply denote the minimum distance by $\lambda$

## A lower bound on the shortest vector
We can first provide a lower bound on the length of the shortest vector of a lattice, given a basis $B$. First, recall the Gram-Schmidt orthognoalization process for converting a basis $B = [\mathbf{b}_1, \mathbf{b}_2, \ldots, \mathbf{b}_n]$ into an orthogonal basis $B^\ast = [\mathbf{b}_1^\ast, \mathbf{b}_2^\ast, \ldots, \mathbf{b}_n^\ast]$ via the following algorithm:

$$
\mathbf{b}_i^\ast \leftarrow \mathbf{b}_i - \sum_{j<i}\frac{\langle \mathbf{b}_i, \mathbf{b}_j^*\rangle}{\langle \mathbf{b}_j^*, \mathbf{b}_j^*\rangle}\mathbf{b}_j^\ast
$$

For convenience, we denote the orthogonalization coefficient by:

$$
\mu_{i, j} = \frac{\langle \mathbf{b}_i, \mathbf{b}_j^*\rangle}{\langle \mathbf{b}_j^*, \mathbf{b}_j^*\rangle}
$$

Also, we define a function $\pi_i$ that projects a vector from $\mathbb{R}^n$ onto the orthogonal basis formed by $\mathbf{b}_i^\ast, \mathbf{b}_{i+1}^\ast, \ldots, \mathbf{b}_n^\ast$:

$$
\pi_i(\mathbf{x}) = \sum_{j\geq i}\frac{\langle \mathbf{x}, \mathbf{b}_j^\ast\rangle}{\langle \mathbf{b}_j^\ast, \mathbf{b}_j^\ast \rangle}\mathbf{b}_j^\ast
$$

Observe that for $i = 1$, $\pi_i$ is the identity function, because the orthogonalized basis still span the same vector space as the original basis. Also, $\pi_i(\mathbf{b}_i) = \mathbf{b}_i^\ast$:

$$
\begin{aligned}
\pi_i(\mathbf{b}_i) &= \sum_{j\geq i}\frac{\langle \mathbf{b}_i, \mathbf{b}_j^\ast\rangle}{\langle \mathbf{b}_j^\ast, \mathbf{b}_j^\ast \rangle}\mathbf{b}_j^\ast \\
&= \sum_{1 \leq j \leq n}\frac{\langle \mathbf{b}_i, \mathbf{b}_j^\ast\rangle}{\langle \mathbf{b}_j^\ast, \mathbf{b}_j^\ast \rangle}\mathbf{b}_j^\ast  - \sum_{j < i}\frac{\langle \mathbf{b}_i, \mathbf{b}_j^\ast\rangle}{\langle \mathbf{b}_j^\ast, \mathbf{b}_j^\ast \rangle}\mathbf{b}_j^\ast \\
&= \pi_1(\mathbf{b}_i) - \sum_{j < i}\frac{\langle \mathbf{b}_i, \mathbf{b}_j^\ast\rangle}{\langle \mathbf{b}_j^\ast, \mathbf{b}_j^\ast \rangle}\mathbf{b}_j^\ast \\
&= \mathbf{b}_i - \sum_{j < i}\frac{\langle \mathbf{b}_i, \mathbf{b}_j^\ast\rangle}{\langle \mathbf{b}_j^\ast, \mathbf{b}_j^\ast \rangle}\mathbf{b}_j^\ast \\
&= \mathbf{b}_i^\ast
\end{aligned}
$$

With these notations established, we can bound $\lambda(\mathcal{L(B)})$ from below using the orthogonalized basis. We will do that by proving the following recursive inequality:

$$
\lambda(\mathcal{L}(\mathbf{b}_1, \mathbf{b}_2, \ldots, \mathbf{b}_n)) \geq \min (\Vert \mathbf{b}_n^\ast \Vert, \lambda(\mathcal{L}(\mathbf{b}_1, \mathbf{b}_2, \ldots, \mathbf{b}_{n-1}))
$$

To prove this inequality, we represent an arbitrary non-zero point on the lattice by $B\mathbf{x} = \sum_{i=1}^n\mathbf{b}_i x_i$:

$$
\begin{aligned}
\Vert B\mathbf{x} \Vert^2 &= \Vert \sum_{i=1}^n\mathbf{b}_i x_i \Vert^2 \\
&= \Vert \sum_{i=1}^{n-1}\mathbf{b}_ix_i + \mathbf{b}_nx_n \Vert^2
\end{aligned}
$$

From here substitute the orthogonalization of $\mathbf{b}_n^\ast = \mathbf{b}_n - \sum_{j<n}\mu_{j, n} \mathbf{b}_j^\ast$:

$$
\begin{aligned}
\Vert B\mathbf{x} \Vert^2 &= \Vert \sum_{i=1}^{n-1}\mathbf{b}_ix_i + \mathbf{b}_nx_n \Vert^2 \\
&= \Vert \sum_{i=1}^{n-1}\mathbf{b}_ix_i + (\mathbf{b}_n^* + \sum_{j<n}\mu_{j, n}\mathbf{b}_j^\ast)x_n \Vert^2 \\
&= \Vert \sum_{i=1}^{n-1}\mathbf{b}_ix_i + (\sum_{j<n}\mu_{j, n}\mathbf{b}_j^\ast) x_n  + \mathbf{b}_n^\ast x_n\Vert^2
\end{aligned}
$$

Observe that by the definition of the orthogonalization process, $\mathbf{b}_n^\ast$ is orthogonal to all other orthogonal basis $\mathbf{b}_1^\ast, \mathbf{b}_2^\ast, \ldots, \mathbf{b}_{n-1}^\ast$, as well as all except the identically indexed original basis $\mathbf{b}_1, \mathbf{b}_2, \ldots, \mathbf{b}_{n-1}$. By Pythagoras theorem we can separate the RHS into orthogonal components:

$$
\begin{aligned}
\Vert B\mathbf{x} \Vert^2 
&= \Vert \sum_{i=1}^{n-1}\mathbf{b}_ix_i + (\sum_{j<n}\mu_{j, n}\mathbf{b}_j^\ast) x_n  + \mathbf{b}_n^\ast x_n\Vert^2 \\
&= \Vert \sum_{i=1}^{n-1}\mathbf{b}_ix_i + (\sum_{j<n}\mu_{j, n}\mathbf{b}_j^\ast) x_n \Vert^2 + \Vert \mathbf{b}_n^\ast x_n \Vert^2 \\
&\geq \Vert \mathbf{b}_n^\ast x_n \Vert^2
\end{aligned}
$$

where $x_n > 0$, the inequality trivially implies $\Vert B\mathbf{x} \Vert \geq \Vert\mathbf{b}_n^\ast\Vert$. where $x_n = 0$, $\mathcal{L}(\mathbf{b}_1, \mathbf{b}_2, \ldots, \mathbf{b}_n) = \mathcal{L}(\mathbf{b}_1, \mathbf{b}_2, \ldots, \mathbf{b}_{n-1})$, so by definition of minimum distance we have $\Vert B\mathbf{x} \Vert \geq \lambda(\mathcal{L}(\mathbf{b}_1, \mathbf{b}_2, \ldots, \mathbf{b}_{n-1}))$. Combining the two cases gives us the overall inequality. $\blacksquare$

Having proved the recursive inequality, we can simply apply recursion $n$ times and arrive at a concrete lower bound for $\lambda$.

$$
\lambda \geq \min_{1 \leq i \leq n} \Vert \mathbf{b}_i^\ast \Vert
$$

## LLL lattice basis reduction algorithm
At the time of this survey, the best algorithm for solving the (approximate) shortest vector problem is the LLL lattice basis reduction algorithm, attributed to Arjen Lenstra, Hendrik Lenstra, and László Lovász. The reduction algorithm transforms an input basis into an LLL-reduced form, where the first base vector of the reduced basis is an approximation of the shortest vector.

**If time permits, we will discuss the algorithm itself. However, I found it okay to first treat the algorithm as a black box, and instead focus on the properties of the reduced basis.**

The definition of a reduced basis is parameterized by a real number $\frac{1}{4} < \delta \leq 1$. A basis $B$ is $\delta$-LLL reduced if the following conditions are satisfied:

- For all $j > i$, $\vert\mu_{j, i}\vert \leq \frac{1}{2}$
- For all $1 \leq i \leq n-1$, $\delta\Vert \pi_i(\mathbf{b}_i)\Vert^2 \leq \Vert\pi_i(\mathbf{b}_{i+1})\Vert^2$

The second clause of the reduced form is of particular interest because we can use it to measure how well $\mathbf{b}_1$ approximates the shortest vector. First observe the RHS of the condition:

$$
\begin{aligned}
\Vert \pi_i(\mathbf{b}_{i+1})\Vert^2 &= \Vert \sum_{j\geq i}\mu_{i+1, j}\mathbf{b}_j^\ast\Vert^2 \\
&= \Vert \mu_{i+1, i}\mathbf{b}_i^\ast + \sum_{j\geq i+1}\mu_{i+1, j}\mathbf{b}_j^\ast\Vert^2 \\
&= \Vert \mu_{i+1, i}\mathbf{b}_i^\ast + \pi_{i+1}(\mathbf{b}_{i+1}) \Vert^2 \\
&= \Vert \mu_{i+1, i}\mathbf{b}_i^\ast + \mathbf{b}_{i+1}^\ast \Vert^2 \\
&= \Vert \mu_{i+1, i}\mathbf{b}_i^\ast \Vert^2 + \Vert \mathbf{b}_{i+1}^\ast \Vert^2 \\
\end{aligned}
$$

Substituting the equation back into the condition:

$$
\delta\Vert \pi_i(\mathbf{b}_i)\Vert^2 \leq \Vert \mu_{i+1, i}\mathbf{b}_i^\ast \Vert^2 + \Vert \mathbf{b}_{i+1}^\ast \Vert^2
$$

Which transforms into:

$$
\Vert \mathbf{b}_i^\ast \Vert^2 \leq \frac{1}{(\delta - \mu_{i+1, i}^2)} \Vert \mathbf{b}_{i+1}^\ast \Vert^2
$$

By the first clause of the reduced basis we know $\mu_{i+1, i}^2 \leq \frac{1}{4}$, which means that $\frac{1}{\delta - \mu_{i+1, i}^2} \leq \frac{1}{\delta - \frac{1}{4}}$. Denote $\alpha = \frac{1}{\delta - \frac{1}{4}}$, then $\alpha > \frac{4}{3}$, and we have the following inequality:

$$
\Vert \mathbf{b}_i^\ast \Vert^2 \leq \alpha \Vert \mathbf{b}_{i+1}^\ast \Vert^2
$$

Note that because $\mathbf{b}_1 = \mathbf{b}_1^\ast$, we can recusively evaluate the inequality above can obtain the following closed inequality:

$$
\Vert \mathbf{b}_1 \Vert^2 \leq \alpha^{i-1} \Vert \mathbf{b}_{i}^\ast \Vert^2 \leq \alpha^{n-1} \Vert \mathbf{b}_{i}^\ast \Vert^2
$$

The inequality above states that $\mathbf{b}_1$ is not longer than $\alpha^{n-1} \Vert \mathbf{b}_{i}^\ast \Vert$ for all $i$, so it must be not longer than the minimum among $\alpha^{n-1} \Vert \mathbf{b}_{i}^\ast \Vert$:

$$
\Vert \mathbf{b}_1 \Vert^2 \leq \min_{1 \leq i \leq n}\alpha^{n-1} \Vert \mathbf{b}_{i}^\ast \Vert^2 = \alpha^{n-1} \min_{1 \leq i \leq n}\Vert \mathbf{b}_{i}^\ast \Vert^2
$$

From the previou section we've derived that $\lambda \geq \min_{1 \leq i \leq n} \Vert \mathbf{b}_i^\ast \Vert$, which we can plug into the inequality above:

$$
\Vert \mathbf{b}_1 \Vert \leq \alpha^\frac{n-1}{2} \lambda
$$

This in equality bounds the length of the first base vector of the reduced basis by some multiples of the minimum distance, where the multiple is exponential with respect to the number of dimensions of the lattice.

In other words, the LLL reduction algorithm computes an approximation of the shortest vector within a factor of $\gamma \in O(\alpha^n)$. With specific choices of "bad" basis and sufficiently high dimension $n$, the basis reduction algorithm will not be able to provide meaningfully tight approximation of the shortest vector, making the shortest vector problem suitably hard for cryptographic applications.

