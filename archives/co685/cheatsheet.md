**table of content**
- [RSA](#rsa)
- [Discrete log](#discrete-log-dh-and-elgamal)

# RSA
RSA was created by Ron Rivest, Adi Shamir, and Leonard Adleman in 1977 (there was an equivalent development by British intelligence in 1973 but it remained classified until 1997).

## RSA encryption scheme ("textbook RSA")
- Key generation
    1. Sample two large prime numbers $p, q$ and compute the public modulus $N = p \cdot q$
    2. Pick a public exponent $e$ (usually 3 or 65535)
    3. Compute [Euclier's totient function](https://en.wikipedia.org/wiki/Euler%27s_totient_function#Euler's_theorem) $\phi(N) = (p-1)(q-1)$
    4. Compute $e$'s multiplicative inverse under modulus $\phi(N)$:  
    $d \leftarrow e^{-1} \mod \phi(N)$
    1. $\texttt{pk} = (N, e), \texttt{sk} = d$
- Encryption:  
Given the plaintext $m \in \mathbb{Z}_N$, return $c \leftarrow m^e \mod N$
- Decryption:  
Given the ciphertext $c \in \mathbb{Z}_N$, return $c^d \mod N$

**Correctness**: Because $d$ is the multiplicative inverse of $e$ under $\phi(N)$, we have $d \cdot e = l \cdot \phi(N) + 1$ for some integer $l$. Because of Euler's theorem $m^{ed} \equiv m^{l \cdot \phi + 1} \equiv (m^\phi)^l \cdot m \equiv 1^l \cdot m \equiv m \mod N$, thus $\texttt{Dec}(\texttt{sk}, \texttt{Enc}(\texttt{pk}, m)) = m$ at all times.

**One-way security**: The RSA assumption states that given $m^e$ it is difficult to recover $m$. The RSA encryption scheme is one-way secure under the RSA assumption.



## RSA signature scheme
The RSA signature scheme is a reverse of the [encryption scheme](#rsa-signature-scheme)

TODO: write this segment

# Discrete log (DH and ElGamal)