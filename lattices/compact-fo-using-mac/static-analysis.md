- [ ] estimate the cost of $\texttt{CPAPKE.decrypt}$
- [ ] estimate the cost of $\texttt{CCAKEM.encap}$ and $\texttt{CCAKEM.decap}$
- [ ] estimate the cost of $\texttt{HMAC}$ and $\texttt{KMAC}$
- [ ] estimate the cost of the modified $\texttt{CCAKEM.encap}$ and $\texttt{CCAKEM.decap}$

# Static analysis of various subroutines of Kyber
We are mainly concerned with the number of Keccak rounds and the number of NTT/inverse NTT calls. Since $\texttt{NTT}$ and $\texttt{NTT}^{-1}$ share identical implementation, we group them together in this static analysis.

## `CPAPKE.encrypt` cost
The pseudocode for `CPAPKE.encrypt` is as follows:

```python
def cpapke_encrypt(pk, m, r):
    """
    pk contains a 256-bit pseudorandom seed (rho) and a vector t in NTT domain
    m is a 256-bit message
    r is a 256-bit pseudorandom seed
    """
    N = 0
    for i in range(KYBER_K):
        for j in range(KYBER_K):
            A[i][j] = xof(pk.rho, i, j)
    
    for i in range(KYBER_K):
        r[i] = sample_cbd(r, N, eta=KYBER_ETA_1)
        N += 1
    
    for i in range(KYEBR_K):
        e_1[i] = sample_cbd(r, N, eta=KYBER_ETA_2)
        N += 1
    
    e_2 = sample_cbd(e, N, eta=KYBER_ETA_2)
    r = NTT(r)
    c_1 = invert_NTT(A.transpose().dot(r)) + e_1
    m = decode(m)
    c_2 = invert_NTT(pk.t.dot(r)) + e_2 + m
    c_1 = compress(c_1)
    c_2 = compress(c_2)
    return (c_1, c_2)
```

There are $k^2$ calls to $\texttt{XOF}$. Each call to $\texttt{XOF}$ needs to absorb $256 + 8 + 8 = 272$ bits, then squeeze $256 \cdot 12 = 3072$ bits. $\texttt{XOF}$ is instantiated using Shake128 with a rate of 1344 bits, so each call incurs 1 round of absorption and $\lceil \frac{3072}{1344} \rceil = 3$ rounds of squeezing for a total of 4 rounds of Keccak permutations.

```python
def genA_cost(k):
    return k * k * (math.ceil(3072 / 1344) + 1)

print(genA_cost(2))  # Kyber512
print(genA_cost(3))  # Kyber768
print(genA_cost(4))  # Kyber1024
```

|security level|Minimum `GenA` Keccak permutations|
|:---|:---|
|Kyber512|16|
|Kyber768|36|
|Kyber1024|64|

This is a minimum because $\texttt{GenA}$ uses rejection sampling and might reject some random bits from the $\texttt{XOF}$ because the value falls outside $\mathbb{Z}_{3329}$


To sample a single coefficient from $\texttt{CBD}_\eta$ we need $2\eta$ bits; to sample a polynomial we need to sample 256 coefficients, so we need a total of $512 \cdot \eta$ bits. $\texttt{PRF}$ is instantiated using Shake256 with a rate of 1088 bits per round, so each call to $\texttt{sampleCBD}_{\eta}$ needs one round of absorption and $\lceil\frac{512\eta}{1088}\rceil$ rounds of squeezing. There are $k$ calls to $\texttt{sampleCBD}_{\eta_1}$ and $k+1$ calls to $\texttt{sampleCBD}_{\eta_2}$:

$$
\text{rounds} = k(\left\lceil\frac{512\eta_1}{1088}\right\rceil + 1) + (k+1)(\left\lceil\frac{512\eta_2}{1088}\right\rceil + 1)
$$

```python
import math

def samplecbd_cost(k, eta1, eta2):
    return (
        k * (math.ceil(512 * eta1 / 1088) + 1) 
        + (k + 1) * (math.ceil(512 * eta2 / 1088) + 1)
    )

print(samplecbd_cost(2, 3, 2))  # Kyber 512
print(samplecbd_cost(3, 2, 2))  # Kyber 768
print(samplecbd_cost(4, 2, 2))  # Kyber 1024
```

|security level|$k$|$\eta_1$|$\eta_2$|$\mathbf{r}, \mathbf{e}_1, e_2$ sampling cost|
|:--|:--|:--|:--|:--|
|Kyber512|2|3|2|12 rounds|
|Kyber768|3|2|2|14 rounds|
|Kyber1024|4|2|2|18 rounds|

Finally, there are $2k + 1$ NTT operations. In summary, the cost of $\texttt{CPAPKE.Encrypt}$ is listed in the table below:

|security level|$k$|$\eta_1$|$\eta_2$|$\texttt{GenA}$ cost|$\texttt{sampleCBD}$ cost|Total Keccak permutations|NTT operations|
|:--|:--|:--|:--|:--|:--|:--|:--|
|Kyber512|2|3|2|16 rounds|12 rounds|28 rounds|5|
|Kyber768|3|2|2|36 rounds|14 rounds|50 rounds|7|
|Kyber1024|4|2|2|64 rounds|18 rounds|82 rounds|9|

## `CPAPKE.decrypt` cost
The decryption costs $2k$ $\texttt{NTT}$ operations. No Keccak permutation is needed.

## `CCAKEM.encap` cost
The encapsulation algorithm calls hash function $H$ and $G$, then calls $\texttt{CPAPKE.encrypt}$.

Hash function $H$ and $G$ are instantiated with $\texttt{SHA3-256}$ (rate 1088 bits) and $\texttt{SHA3-512}$ (rate 576 bits) respectively.

## `CCAKEM.decap` cost
