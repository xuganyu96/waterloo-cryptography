# The FO Transformation
- Describe the inputs: asym, sym, KDF $G$, hash $H$
- Describe the encryption routine:
    - Sample random $\sigma$
    - Symmetric side: derive symmetric key, encrypt input message
    - Asymmetric side: derive coin $r \leftarrow H(c, \sigma)$, encrypt $\sigma$
    - Output $(e, c)$
- Describe the decryption routine:
    - Decrypt $e$ to get $\hat{\sigma}$
    - Derive $\hat{r}$, re-encrypt $\hat{\sigma}$, check integrity
    - Derive $\hat{a}$, decrypt $c$

## Security result
> For every IND-CCA adversary against the hybrid scheme, there exists a OW-CPA adversary against the underlying PKE and an IND-CPA adversary against the underlying symmetric ciphert such that

$$
\epsilon^\text{hy}_\text{IND-CCA} 
    \leq q_D2^{-\gamma} 
        + q_H\epsilon^\text{asym}_\text{OW-CPA} 
        + \epsilon^\text{sym}_\text{IND-CPA}
$$

## Security proof overview:
Sequence of games. The overall goal is to use the OW-CPA adversary and the IND-CPA adversary to simulate the IND-CCA game. There are two main challenges:

1. Need to simulate a decryption oracle
2. The OW-CPA game encrypts $m^\ast$ using a truly random coin, but FO transformation uses a pseudorandom coin $r \leftarrow H(c, \sigma)$
3. The IND-CPA game encrypts $m_b^\ast$ using a truly random key, but FO transformation uses a pseudorandom key $a \leftarrow G(\sigma)$

## simulating decryption oracle
Under the random oracle model, decryption oracle can be simulated without the secret key.

- $\mathcal{O}^D$ inputs are $(\tilde{e}, \tilde{c})$
- Check if $H$ has a record $(c, \sigma, r)$ such that $c = \tilde{c}$ and $E(\sigma, r) = \tilde{e}$
    - If yes, $a \leftarrow G(\sigma), m \leftarrow D^\text{sym}_a(\tilde{c})$, output $m$
    - If no, reject with $\bot$

## true randomness vs pseudorandomness
When encrypting the IND-CCA challenge ciphertext:

1. $b \leftarrow \{0, 1\}$
2. Sample truly random $a^\ast, r^\ast$ instead of using $G$ or $H$

## sequence of games
Loss of tightness when simulating decryption oracle without secret key, and when replacing pseudorandom $a^\ast, r^\ast$ with truly random values, but the loss is negligible.

- Game 0: IND-CCA
- Game 1: modify decryption oracle
- Game 2: replace $a^\ast, r^\ast$

## security bounds
From game 0 to game 1, we lose tightness when IND-CCA adversary constructs valid ciphertext without using the hash function $H$. Under the random oracle model, the IND-CCA adversary cannot do better than guessing a random ciphertext, hence $q_D2^{-\gamma}$

From game 1 to game 2, we lose tightness when IND-CCA adversary queries $\sigma^\ast$ on the hash function $H$ or $G$

The probability that the IND-CCA adversary queries $\sigma^\ast$ can be bounded by the advantage of an OW-CPA adversary: the OW-CPA adversary picks a random hash query as the return value

The advantage of the IND-CCA adversary in game 2 can be bounded by the advantage of an IND-CPA adversary against the symmetric cipher

# FO 1999 comments
- $q_H \cdot \epsilon^\text{asym}_\text{OW-CPA}$ is not tight
- What if there is decryption error
- KEM has different security requirements

# 2017 result
A modular construction:
1. From OW-CPA/IND-CPA PKE to OW-PCVA PKE, with tight reduction if PKE is IND-CPA
2. OW-PCVA PKE to IND-CCA KEM

## What is PCVA?
Adversary has access to public key, hash oracles, and **two additional oracles**:

- Plaintext checking oracle
- Ciphertext validity oracle

### Plaintext checking oracle
Plaintext-checking oracle (PCO): For a fixed keypair, checks if the queried plaintext-ciphertext pair $(\tilde{m}, \tilde{c})$ is valid.

Vanilla implementation:
- Use secret key to decret $\tilde{c}$ and compare with $\tilde{m}$
- Encrypt $\tilde{m}$ and compare with $\tilde{c}$

### Ciphertext validity oracle
Ciphertext-validity oracle (CVO): for a fixed keypair, checks if the queried ciphertext $\tilde{c}$ will be rejected.

Vanilla implementation:
- Uses secret key to decrypt $\tilde{c}$ into $\hat{m}$
- Encrypt $\hat{m}$ and compare with $\tilde{c}$

## OW-PCVA transformation
Inputs
- A PKE $(\operatorname{KeyGen}, E, D)$
- A hash function $G: \mathcal{M} \rightarrow \text{Coin}$

Encrypt $E^T(\text{pk}, m)$:
- $r \leftarrow G(m)$
- $c \leftarrow E(\text{pk}, m)$
- Return $c$

Decrypt $D^T(\text{sk}, c)$
- $\hat{m} \leftarrow D(\text{sk}, m)$
- Check if $E(\text{pk}, \hat{m}, G(\hat{m})) = c$; if not, output $\bot$
    - If equal, output $\hat{m}$
    - If not equal, output $\bot$

## OW-PCVA security result
For every OW-PCVA adversary against the T transformation $(E^T, D^T)$ with advantage $\epsilon^T_\text{OW-PCVA}$, where exists an IND-CPA adversary against the underlying PKE with advantage $\epsilon_\text{IND-CPA}$ such that

$$
\epsilon^T_\text{OW-PCVA} \leq q_V2^{-\gamma} + \frac{1}{\vert\mathcal{M}\vert} + 3\epsilon_\text{IND-CPA}
$$

note: the discussion of decryption error is omitted for clarity, but decryption error contributes a negligible loss of tightness if the probability of decryption error is negligible

## Security proofs overview
Similar strategy to what's used in the 1999 FO transformation.

- Modify PCO and CVO so they don't need to use secret keys
- Simulate OW-PCVA game with an IND-CPA adversary

## Modified PCO
Given the query $(\tilde{m}, \tilde{c})$, instead of using secret key to decrypt, check the tape of hash $G$: is there a record $(m, r)$ such that
1. $m = \tilde{m}$
1. $E(m, r) = \tilde{c}$

## Modified CVO
Given query $\tilde{c}$, instead of using secret key to decrypt, check the tape of hash $G$: is there a record $(m, r)$ such that
1. $m = \tilde{m}$
1. $E(m, r) = \tilde{c}$

## Sequence of games
1. Game 0 is OW-PCVA game
1. Game 1 uses the modified PCO and CVO
2. Game 2 uses a truly random coin to encrypt challenge ciphertext instead of a pseudorandom coin

## Security bounds
From game 0 to game 1, we lose security when $\mathcal{A}^T_\text{OW-PCVA}$ produces valid ciphertext without querying $G$. Under the random oracle model, the adversary cannot do better than guessing a random ciphertext. Hence $q_V2^{-\gamma}$

From game 1 to game 2, we close security when $\mathcal{A}^T_\text{OW-PCVA}$ queries $G$ on the challenge plaintext $m^\ast$.

Game 2 can be perfectly simulated by an OW-CPA adversary, so the probability of winning game 2 is bounded by the OW-CPA adversary's advantage, which in turn is bounded by an IND-CPA adversary's advantage $\epsilon_\text{OW-CPA} = \frac{1}{\vert\mathcal{M}\vert} + \epsilon_\text{IND-CPA}$.

The probability of $\mathcal{A}^T_\text{IND-CCA}$ querying $m^\ast$ can be bounded by the advantage of an IND-CPA adversary attacking the underlying PKE. The IND-CPA adversary checks the tape of $G$ to see if there is any $(m, r)$ that matches one of $(m_0, m_1)$: if there is return the match, if there is no, return a coin toss

## KEM: note on omission
Hofheinz et al proposed 4 variations depending on:

||explicit rejection|implicit rejection|
|:--|:--|:--|
|PKE is IND-CPA|$U^\bot$|$U^{\cancel{\bot}}$|
|PKE is OW-CPA|$U^\bot_m$|$U^{\cancel{\bot}}_m$|

For simplicity we limit discussion to explicit rejection and IND-CPA PKE $U^\bot$

## KEM: construction
Encapsulation $E^{U^\bot}(\text{pk})$:
- Sample random message $m \leftarrow \mathcal{M}$
- Encrypt with OW-PCVA PKE $c \leftarrow E^T(\text{pk}, m)$
- Derive key $K \leftarrow H(m, c)$
- Output $(c, K)$

Decapsulation $D^{U^\bot}(\text{sk}, c)$
- Decrypt with OW-PCVA PKE $m \leftarrow D^T(\text{sk}, c)$  
This step already checks integrity
- Output $H(m, c)$

## Security result
For every IND-CCA adversary against $U^\bot$ KEM with advantage $\epsilon^{U^\bot}_\text{IND-CCA}$, there exists an OW-PCVA adversary against $(E^T, D^T)$ with advantage $\epsilon^T_\text{OW-PCVA}$ such that

$$
\epsilon^{U^\bot}_\text{IND-CCA} \leq \epsilon^T_\text{OW-PCVA}
$$

## Simulating decapsulation oracle
There are two tricks to simulate decapsulation oracle without secret key:
1. Use PCO and CVO
2. Make the decapsulation oracle stateful

The modification includes both $H$ and $\mathcal{O}^D$
- $H$ behaves just like a random oracle, except that when queried on $(\tilde{m}, \tilde{c})$, it also checks PCO. If $(\tilde{m}, \tilde{c})$ is valid, then $(\tilde{c}, \tilde{K})$ is added to the tape of the decapsulation oracle
- When $\mathcal{O}^D$ is queried on $\tilde{c}$, it first checks CVO and reject invalid ciphertext accordingly. Then it checks its tape
    - If there is a matching $\tilde{K}$, then return the matching value
    - If there is not, generate a random $\tilde{K}$, add $(\tilde{c}, \tilde{K})$

**This pair of $(H, \mathcal{O}^D)$ will behave exactly like their vanilla implementations**

## Sequence of games
- Game 0: IND-CCA KEM
- Game 1: Use modified hash and decapsulation oracle
- Game 2: Modify $H$ so that if queried on $(m^\ast, c^\ast)$ by the adversary, returns another random value than $K^\ast$

## Security bounds
Game 0 and game 1 are exactly the same

From game 1 to game 2, we lose security when the IND-CCA adversary queries on the challenge values

Game 2 is unwinnable, no adversary can have any advantage

The probability of IND-CCA adversary winning can be bounded by the advantage of an OW-PCVA adversary. $\mathcal{A}_\text{OW-PCVA}$ checks the tape of $H$ to see if there is a record $(m, c, K)$ such that $c = c^\ast$
