# Zero-knowledge proof based on Syndrome Decoding Problem

- **Commit 1**:  
  $\mathbf{u} \stackrel{\$}{\leftarrow}\mathbb{F}_q^n$, $\sigma \stackrel{\$}{\leftarrow}S_n \times (\mathbb{F}_q^\ast)^n$, $c_1 \leftarrow G(\sigma, H\mathbf{u})$, $c_2 \leftarrow G(\sigma(\mathbf{u}), \sigma(\mathbf{e}))$. Send $(c_1, c_2)$
- **Challenge 1:**  
  $\alpha \stackrel{\$}{\leftarrow} \mathbb{F}_q$
- **Commit 2:**  
  $\vec{\beta} \leftarrow \sigma(\mathbf{u} + \alpha \mathbf{e})$
- **Challenge 2:**  
  $b \stackrel{\$}{\leftarrow} \{0, 1\}$
- **Response:**
    - If $b = 0$, send $\sigma$
    - If $b = 1$, send $\sigma(\mathbf{e})$
- **Verification:**
    - If $b = 0$, check that $G(\sigma, H\sigma^{-1}(\vec{\beta}) - \alpha H \mathbf{e}) = c_1$
    - If $b = 1$, check that $\mathop{wt}(\sigma(\mathbf{e})) = w$ and that $G(\mathbf{z} - \alpha\mathbf{e}, \sigma(\mathbf{e})) = c_2$