# Authenticated encryption (in symmetric cipher)
The indistinguishability game is well defined. We now define the game for ciphertext integrity under chosen-plaintext attack:

1. Challenger samples a random key $k^\ast$
2. Adversary can issue encryption queries: for each plaintext queried, the oracle returns the encryption of the queried plaintext under the challenger's key
3. Adversary outputs a ciphertext $c^\ast$

The adversary wins the CI-CPA game iff $D(k^\ast, c^\ast) \neq \bot$, or in other words, the output ciphertext is not rejected by the decryption routine under the challenger's key.

> A symmetric cipher $(E, D)$ over $(\mathcal{K}, \mathcal{M}, \mathcal{C})$ is **authenticated encryption** if it is both IND-CPA secure and CI-CPA secure

There is a variant called one-time authenticated encryption, which is suitable for when each symmetric key is used to encryption a single message in its lifetime. This is realistic scenario, and is a meaningful variation because some one-time MAC is more efficient than many-time MAC.

The key result about AE is that it implies IND-CCA security

> If a symmetric cipher is authenticated encryption, then it is IND-CCA secure.

More specifically, for every IND-CCA adversary that issues $q_D$ decryption queries and that has advantage $\epsilon_\text{CCA}$, there exists a IND-CPA adversary with advantage $\epsilon_\text{CPA}$ and CI-CPA adversary with advantage $\epsilon_\text{CI}$ such that

$$
\epsilon_\text{CCA} \leq \epsilon_\text{CPA} + q_D \cdot \epsilon_\text{CI}
$$