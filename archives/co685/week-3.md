# Week 1-3 review
Up to week 3 there are two ciphers and a few security definitions.

- Ciphers
    - ElGamal
    - RSA
- Security models
    - OW-CPA
    - IND-CPA
    - IND-CCA1, IND-CCA2

## ElGamal
`PGen` generates the **base number $g$** and **prime number $p$** that defines the plaintext and ciphertext space $\mathbb{Z}_p$. `KeyGen` generates the **secret key $x \in \mathbb{Z}$** and the **public key $g^x \mod p$**.

To encrypt message $m \in \mathbb{Z}_p$, first generate some $y \in \mathbb{Z}$, then the ciphertext is $(c_1, c_2)$, where $c_1 = g^y$ and $c_2 = m \cdot (g^x)^y = m \cdot g^{xy}$. To decrypt the message, we need to use the secret key $x$:

$$
c_1^{-x}c_2 = g^{-xy} \cdot m \cdot g^{xy} = m
$$

## RSA
Parameter generation in RSA is trivial.

For key generation, there are the following steps:

1. Generate large prime $p, q$ and compute their product $n = p \cdot q$. Both primes should be kept secret
2. Compute Euler's totient function $\phi(n) = (p-1)(q-1)$
3. Generate a number $e$, where $e$ is co-prime with $\phi(n)$. **the public key is $(e, n)$**
4. Compute the multiplicative inverse of $e$ in $\mathbb{Z}_{\phi(n)}$, denoted by $d$. **The secret key is d**

To encrypt $m \in \mathbb{Z}$ using the public key $e$:

$$
c = m^e \mod n
$$

To decrypt the ciphertext:

$$
m^\prime = c^d \mod n
$$

Because $d$ is the multiplicative of $e$ we know that $e \cdot d \equiv 1 \mod \phi(n)$, meaning that for some integer $k$, $e \cdot d = k \cdot \phi(n) + 1$.

From here we know that

$$
\begin{aligned}
c^d &= (m^e)^d \mod n \\
&= m^{ed} \mod n \\
&= m^{\phi(n) \cdot k + 1} \mod n \\
&= m \cdot (m^{\phi(n)})^k \mod n
\end{aligned}
$$

For when $m$ and $n$ are relatively prime, we can use Euler's theorem $m^\phi \equiv 1 \mod n$ to show that the decryption is correct. For when $m$ and $n$ are not relatively prime, we can use Fermat's little theorem to argument the congruence to be true in general.

## Security definitions
In modern public key cryptography, the security definitions are usually made using a sequence of games. The common components involve the following:

1. A parameter generation function $\text{PGen}$
2. A key generation function $\text{KeyGen}$ that involves a security parameter $\lambda$ so that the complexity and security of the key increases alongside the security parameters

### OW-CPA
In the OW-CPA game, the challenger will encrypt a randomly sampled message. The adversary has access to the public key and can thus encrypt arbitrary plaintext. The adversary is challenged to output some plaintext from the challenge ciphertext, and wins if the output plaintext is equal to the challenge plaintext

### IND-CPA
In the IND-CPA game, the adversary generates two distinct message $m_0, m_1$ and gives the "chosen plaintext" to the challenger. The challenger randomly chooses one of the message, encrypts it, then returns the challenge ciphertext to the adversary. The adversary then guesses using the challenge ciphertexts, which of the plaintexts is encrypted.

Any deterministic encryption scheme is necessarily NOT IND-CPA, since the adversary can trivially encrypt both plaintexts using the public key. Textbook RSA (as shown above) is not IND-CPA. ElGamal is IND-CPA under the DDH (but there is a catch with some unfortunate choices of $g$ and $p$)

### IND-CCA, IND-CCA2
In the IND-CCA game, the adversary can query the challenger for the decryption of a limited number of ciphertexts. **After the queries are made**, the adversary generates two plaintexts, the challenger randomly chooses one of them to encrypt, and the adversary wins by correctly guessing which plaintext is encrypted

In the IND-CCA2 game, the adversary first generates the plaintexts and receives the randomly chosen encryption, then makes queries to the challenger. Note that this necessarily means that the challenger will refuse to decrypt the challenge ciphertext. The adversary wins by correctly guessing the choice of plaintext encrypted.

It is worth observing that if changes in ciphertext can be predictably reflected in the plaintext (and vice versa), then IND-CCA/CCA2 can be broken. Therefore, any encryption that exhibits homomorphic properties is automatically not CCA/CCA2 secure.

### Security of RSA and ElGamal

||OW-CPA|IND-CPA|IND-CCA|IND-CCA2|
|:--|:--|:--|:--|:--|
|RSA|Yes, under the RSA assumption|No|No|No|
|ElGamal|Yes, under the computational DH assumption|Yes, under the decisional DH assumption *|Maybe|No|

There is a catch for the decisional DH assumption: from Fermat's little theorem we know that for some $g$ and prime number $p$, $g^{p-1} \equiv 1 \mod p$, so the square root of $g^{p-1}$ is either 1 or -1. Observe that the product of two randomly sampled integers has a 75% change of being even, but a randomly sampled integer only has 50% chance. We can use these two facts to gain a substantial advantage in the DDH game for unfortunate choices of $g$. In practice, however, $g$ and $p$ are usually fixed to some carefully chosen value so that this kind of attack is mitigated.