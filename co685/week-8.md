## Security of full-domain hash RSA signature
In a FDH RSA signature scheme, the signing and verification functions work as follows:

$$
\begin{aligned}
\text{pk} &= (n, e) \\ \text{sk} &= d \\
\text{Sign}(\text{pk}, \text{sk}, m) &= H_n(m)^d \mod n \\
\text{Verify}(\text{pk}, m, \sigma) &= \begin{cases}
1 \; \text{, if } \sigma^e \equiv H_n(m) \mod n \\
0 \; \text{otherwise}
\end{cases}
\end{aligned}
$$

**We can show that FDH RSA is secure under the RSA assumption.**

First recall the RSA assumption: Given $(n, e), x$ it is infeasible to compute $x^d$, where $d$ is the multiplicative inverse of $e$ modulo $n$. We will show the security by showing that if there exists an FDH-RSA adversary who can existentially forge signatures, then we can build an RSA adversary who can compute $x^d$.

The key idea in the construction of $\mathcal{A}_\text{RSA}$ is that existentially forging signature in FDH-RSA is equivalent to finding a pair $(m, \sigma)$ such that $\sigma^e = H_n(m)$, which means that the FDH-RSA adversary needs to query the random oracle $H_n$ that hashes the message into the group element in $\mathbb{Z}_n^*$. **If the adversary can substitute the random oracle with a constructed hash function that is indistinguishable from truly random, then the FDH-RSA adversary will retain its advantage while giving something that helps the RSA adversary compute $x^d$.**

In this case, the adversary replaces $H_n$ with $H^\prime_n: m \mapsto r^ex$ for some uniformly sampled $r \leftarrow \mathbb{Z}_n^*$. When the EF-CMA adversary returns the signature $\sigma$ we know that $\sigma^e = r^ex$, from here the RSA adversary can compute:

$$
\begin{aligned}
x^d &= ((\frac{\sigma}{r})^e)^d \\
&= (\frac{\sigma}{r})^{ed} \\
&\equiv \frac{\sigma}{r} \mod n
\end{aligned}
$$

It remains to show that $\mathcal{A}_\text{EF-CMA}$ still works as intended, which means that $\mathcal{A}_\text{RSA}$ also needs to play the role of the signing oracle for chosen messages and provide hash value of the chosen messages.

When $\mathcal{A}_\text{EF-CMA}$ requests the signature of a chosen message $m_i$, $\mathcal{A}_\text{RSA}$ returns a randomly sampled $\sigma_i \leftarrow \mathbb{Z}_n^*$ and takes notes that the hash of $m_i$ must be $\sigma_i^e$ if queried. This maintains the indistinguishability between the constructed hash function and an actual random oracle.

There is also the possibility that $\mathcal{A}_\text{EF-CMA}$ requests the hash of $q$ messages (among which is the chosen message) before making queries for their signatures. In this case, $\mathcal{A}_\text{RSA}$ simply takes a blind guess which of the $q$ hash queries is for the forgery message and returns the special hash $r^ex$; otherwise it returns random hash $\sigma_i^e$. If $\mathcal{A}_\text{RSA}$ gusses correctly, then it can use the forged signature to correctly compute $x^d$; otherwise the computation would be incorrect. In this case the advantage of $\mathcal{A}_\text{RSA}$ is $\frac{1}{q}$ of the advantage of $\mathcal{A}_\text{EF-CMA}$, which is still non-negligible because $q$ is polynomially bounded