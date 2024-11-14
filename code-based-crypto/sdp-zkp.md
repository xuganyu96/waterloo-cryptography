# Zero-knowledge proof based on the Syndrome Decoding Problem
Cayrel, Veron, and Alaoui proposed in 2010 the following zero-knowledge proof in which the prover tries to demonstrate knowledge of the solution $\mathbf{e}$ to a Syndrome Decoding Problem (SDP): 

$$
\mathbf{y} = H\cdot\mathbf{e}
$$

Each instance is parameterized by a $(n, k)$-linear code over some finite field $K$, some chosen Hamming weight $w$. The procedure is as follows:

- **Key generation:**
    - sample random $\mathbf{e} \in K^n$ with Hamming weight $w$
    - sample random full-rank matrix $H \in K^{(n-k)\times n}$
    - compute $\mathbf{y} \leftarrow H\cdot\mathbf{e}$
    - $\texttt{sk} \leftarrow \mathbf{e}, \texttt{pk} \leftarrow (\mathbf{y}, H)$