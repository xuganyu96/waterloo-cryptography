# Zero-knowledge proof based on the Syndrome Decoding Problem
Cayrel, Veron, and Alaoui proposed in 2010 the following zero-knowledge proof in which the prover tries to demonstrate knowledge of the solution $\mathbf{e}$ to a Syndrome Decoding Problem (SDP): 

$$
\mathbf{s} = H\cdot\mathbf{e}
$$

Each instance is parameterized by a $(n, k)$-linear code over some finite field $K$, some chosen Hamming weight $w$. The procedure is as follows:

- **Key generation:**
    - sample random $\mathbf{e} \in K^n$ with Hamming weight $w$
    - sample random full-rank matrix $H \in K^{(n-k)\times n}$
    - compute $\mathbf{s} \leftarrow H\cdot\mathbf{e}$
    - $\texttt{sk} \leftarrow \mathbf{e}, \texttt{pk} \leftarrow (\mathbf{s}, H)$
- **Commit**:
    - Sample random word $\mathbf{u} \leftarrow K^n$ and compute its syndrome $\mathbf{w} \leftarrow H\cdot\mathbf{u}$
    - Sample random isometry $\sigma \leftarrow S_n \times (K^\ast)^n$
    - Compute $c_0 \leftarrow h(\sigma, H\cdot\mathbf{u})$ and $c_1 \leftarrow h(\sigma(\mathbf{u}), \sigma(\mathbf{e}))$
    - Return $c = (c_0, c_1)$
- **Challenge 1**: Sample random $\beta \stackrel{r}{\leftarrow} K^\ast$
- **Response 1**: Return $\mathbf{y} \leftarrow \sigma(\mathbf{u} + \beta\cdot\mathbf{e})$
- **Challenge 2**: Sample random bit $b \stackrel{r}{\leftarrow} \{0, 1\}$
- **Response 2**:
    - If $b = 0$, return $\sigma$
    - If $b = 1$, return $\sigma(\mathbf{e})$
- **Verification**:
    - If $b = 0$, accept if and only if $c_0 = h(\sigma, H\sigma^{-1}(\mathbf{y}) - \beta\mathbf{s})$
    - If $b = 1$, accept if and only if $c_1 = h(\mathbf{y} - \beta\sigma(\mathbf{e}), \sigma(\mathbf{e}))$ and if $\mathop{\text{wt}}(\sigma(\mathbf{e})) = w$

The following theorem is due to the original authors:

> This protocol is complete and zero-knowledge. It is sound with soundness error $\epsilon = \frac{q}{2(q-1)}$ where $q = \vert K \vert$. Under the random oracle model, if there exists a cheating prover with non-negligible advantage over the soundness error, then there exists a second PPT that can either find collision in the hash function, or solve the syndrome decoding problem.