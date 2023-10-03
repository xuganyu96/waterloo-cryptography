# Naive LWE public-key cryptography system
The naive public-key cryptography system is described in [this survey](https://cims.nyu.edu/~regev/papers/lwesurvey.pdf).

There are four components to this cryptographic system:

## PGen and KeyGen
In parameter generation we take the security parameter $n \in \mathbb{Z}$ and return the following items (their values are taken from the survey; $\log$ means $\log_2$):

1. The **modulo** $q$, a prime number between $n^2$ and $2n^2$
1. The **sample size** $m = 1.1 \cdot n\log q$
1. The **noise factor** $\alpha = (\sqrt{n}\log^2n)^{-1}$

Then, for key generation, first generate the secret key $\vec{s}$ then take $m$ samples from the LWE distribution:

$$
\begin{aligned}
\text{sk} &= \vec{s} \leftarrow\$ \mathbb{Z}_q^n \\
\text{pk} &= (A, \vec{b}) = (A, A\vec{s} + \vec{e})
\end{aligned}
$$

Where $A \leftarrow\$\mathbb{Z}_q^{m \times n}$ is uniformly sampled, and $\vec{e}$ is sampled from an error distribution $\mathcal{X} = N(\mu=0, \sigma=\alpha\cdot q)$


## Encryption and decryption
The message space is a bit stream $\mathcal{M} = \{0, 1\}^l$ where $l$ is the number of bits. Encryption and decryption are performed per bit of the plaintext.

Let $x \in \mathcal{M}$ be a message and let $x[i]$ denote the bit at index $i$. To encrypt $x[i]$, pick a random subset of the LWE samples and sum them into a single new sample. If the plaintext bit is 1, then add an additional $\lfloor \frac{q}{2} \rfloor$:

$$
\begin{aligned}
E(\text{pk}, x[i]) &= (\vec{c}_1, c_2) \\
&= (\sum_{j\in S} \vec{a}_j,( \sum_{j \in S}b_j) + \lfloor \frac{q}{2} \rfloor \cdot c[i])
\end{aligned}
$$

Where $S \subseteq \{1, 2, \ldots, m\}$ is a random non-empty subset of the LWE samples.

For decryption, we subtract the inner product of $c_1$ and $s$ from $c_2$ and check if the result is closer to $0$ or $\lfloor\frac{q}{2}\rfloor$:

$$
\begin{aligned}
x^\prime &= c_2 - \langle\vec{c_1}, \vec{s}\rangle \\
D(\text{sk}, (\vec{c}_1, c_2)) &= \begin{cases}
1, x^\prime\text{ is closer to }\lfloor\frac{q}{2}\rfloor \\
0, \text{otherwise}
\end{cases}
\end{aligned}
$$

The correctness of this naive-LWE system relies on the observation that the sum of $m$ error terms has negligible chance of being greater than $\frac{q}{4}$ when $e \leftarrow N(0, \alpha q)$.