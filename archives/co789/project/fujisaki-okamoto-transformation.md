The Fujisaki-Okamoto transformation takes an IND-CPA asymmetric encryption scheme and an IND-CPA symmetric encryption scheme, as well as two hash functions, and produces an asymmetric encryption scheme with IND-CCA2 security. The transformed scheme is called a hybrid scheme for clarity.

The **key generation** of the hybrid scheme is identical to that of the asymmetric scheme. The public key and secret key are respectively denoted by $\text{pk}, \text{sk}$.

**The message space of the hybrid scheme is identical to the message space of the symmetric scheme**. Given $m \in \mathcal{M}_{\text{sym}}$, the first part of the hybrid ciphertext is the encryption of $m$ using the symmetric encryption routine, but we need a symmetric key. This symmetric key is derived by hashing a random sample from the asymmetric scheme's message space: first sample $\sigma \leftarrow \mathcal{M}_{\text{asym}}$, then compute $a \leftarrow G(\sigma)$, finally:

$$
c_1 \leftarrow \text{Enc}^\text{sym}_a(m)
$$

This means that $G$ is a hash function that hashes into the key space of the symmetric scheme $G: \{0, 1\}^\ast \rightarrow \mathcal{K}^{\text{sym}}$.

Since $\sigma$ is the key material for the symmetric encryption, it needs to be "protected", so we also encrypt it using the asymmetric public key. Here, the asymmetric encryption scheme requires a source of "randomness", which is supplied by $h \leftarrow H(\sigma, c_1)$. This gives us the specification of the second hash function H:

$$
H: \{0, 1\}^\ast \times \{0, 1\}^\ast \rightarrow \text{Coin}^\text{asym}
$$

The second part of the encryption is thus:

$$
c_2 \leftarrow \text{Enc}_\text{pk}^\text{asym}(\sigma, h)
$$

Finally, output the hybrid ciphertext $c \leftarrow (c_1, c_2)$.

When decrypting, first use the asymmetric secret key to recover the symmetric key material: $\hat{\sigma} \leftarrow \text{Dec}_\text{sk}^\text{asym}(c_2)$. Check whether $\hat{\sigma} \in \mathcal{M}^\text{asym}$: if yes, recover the symmetric key $\hat{a} = G(\sigma)$; otherwise, terminate the decryption routine and output the error symbol $\bot$.

We need to further check that $\hat{\sigma}$ is indeed the correct key material. First, recover the "nonce" of the public key encryption scheme $\hat{h} \leftarrow H(\hat{\sigma}, c_1)$, then re-encrypt $\hat{\sigma}$ under the public key and the recovered nonce: $\hat{c}_2 \leftarrow \text{Enc}_\text{pk}^\text{asym}(\hat{\sigma}, \hat{h})$. If the re-encryption checks out $\hat{c}_2 = c_2$, then use the recovered symmetric key to reover the message $\hat{m} = \text{Dec}_{\hat{a}}^\text{sym}(c_1)$. Otherwise, output the error symbol $\bot$.