# Performance measurement
On June 14 my colleagues reported performance measurements of our initial design. Kyber implementation is taken from the reference implementation submitted to round 3. We chose KMAC to be the message authentication code. Runtime cost is measured in CPU cycles. Ciphertext and tag sizes are measured in bytes.

||Encapsulation|Decapsulation|`CPAPKE.Encrypt`|`CPAPKE.Decrypt`|KMAC|Original ciphertext size|tag size|
|:--|:--|:--|:--|:--|:--|:--|:--|
Kyber512|23230|17594|10165|825||768||
Kyber512 w/MAC|29166|14089|||8359||48|
Kyber768|33055|22650|19279|1045||1088||
Kyber768 w/MAC|41077|17747|||10619||64|
Kyber1024|43893|33827|24891|1338||1568||
Kyber1024 w/MAC|77011|27857|||13306||96|

The decrease in decapsulation runtime is minimal, while the increase in encapsulation runtime is substantial. There are many optimizations:

1. KMAC is implemented using Shake128/Shake256, which is known to be computationally expensive (between 5 and 10 cycles per byte of input data). A MAC based on polynomial and binary finite field (such as what's used in AES-GCM or ChaCha20-Poly1305) should be more efficient.
1. Kyber's round 3 uses the variation of modular Fujisaki-Okamoto transformation that outputs the shared key $K = H(m, c)$ as the hash of both the plaintext and the ciphertext. In fact, it is $K = H(m, H(c))$, so the hash of the ciphertext is already computed. We might be able to compute the tag on the hash of the ciphertext (32 bytes) instead of the ciphertext (more than 1000 bytes) itself.

We considered some additional strategies, such as using "encrypt-and-mac" and "mac-then-encrypt". The logistics problem of "mac-then-encrypt" aside, both strategies have the problem that the tag is not computed on the ciphertext, so the malleability of many lattice-based ciphertext means that a decryption oracle will trivially decrypt a tempered ciphertext.