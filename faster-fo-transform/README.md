# Faster Fujisaki-Okamoto transformation
The manuscript can be found on [Overleaf](https://www.overleaf.com/read/qgqctkbwyskm#ef31da).

## Introduction
The 1999 Fujisaki-Okamoto transformation (FO transform for short) and the 2017 modular transformation (modular FO transform for short) take a public-key encryption scheme that is OW-CPA (or IND-CPA) secure and uses **de-randomization** and **re-encryption** to add ciphertext integrity. However, public-key encryption is usually a computationally expensive routine, so running the encryption routine as a sub-routine of the decryption routine leaves much computational efficiency to be desired. As the authors of the modular FO transform stated in their paper:

> It is an interesting open problem to come up with alternative transformations that get rid of derandomization or that dispense with re-encryption (which preserving efficiency)

In this project, we propose to replace re-encryption with a message authentication code (MAC). Given an OW-CPA secure encryption scheme, we perform an alternative transformation:

1. In the encryption routine, we derive a MAC key from the plaintext message, then sign the unauthenticated ciphertext using the MAC key. The ciphertext-tag pair is returned as the authenticated ciphertext.
2. In the decryption routine, the unauthenticated ciphertext is first decrypted, and the decryption is used to re-derive the MAC key. We then use the re-derived MAC key to verify the tag against the ciphertext: if the tag is valid, return the decryption, otherwise reject the ciphertext as invalid.

It's easy to see that if both Alice and Bob are honest, then the transformed scheme is correct. If the MAC is an ideal PRF, then it should not help any passive adversary with recovering the message. Finally, if the ciphertext is tempered with, then the tag will change unpredictably without knowing the MAC Key, so assuming the MAC to be unforgeable, the adversary cannot produce new valid ciphertexts without recovering the MAC key first. The key security reduction is as follows:

> If the input public-key encryption scheme is OW-CPA secure, and the MAC is existentially unforgeable, then the transformed scheme is OW-PCVA secure.

From here, it is easy to use the results from the modular FO transform to construct IND-CCA secure key encapsulation mechanism (KEM).

## Adding ciphertext integrity
The modular FO transform is **modular** because the authors broke the full KEM construction into two steps. The first step is called the $\texttt{T}$ transformation, which takes a OW-CPA secure public-key encryption scheme and transforms it into a OW-PCVA secure public-key encryption scheme by adding mechanism for ensuring ciphertext integrity. The second step is called the $\texttt{U}$ transformation, which takes an OW-PCVA secure public-key encryption scheme (or OW-PCA if using implicit rejection) and outputs an IND-CCA secure KEM.

We propose an alternative $\texttt{T}$ transformation that also achieves OW-PCVA security. Thanks to the modularity of the modular FO transform, the alternative transformation can plug right into the existing construction and produce an IND-CCA secure KEM.

> I have a crazy, probabilistic $(\texttt{E}_\texttt{EtM}, \texttt{D}_\texttt{EtM})$ that might work. This is a work in progress


## Performance with KMAC
On June 14 my colleagues reported performance measurements of our initial design. Kyber implementation is taken from the reference implementation submitted to round 3. We chose KMAC to be the message authentication code. Runtime cost is measured in CPU cycles. Ciphertext and tag sizes are measured in bytes.

||Encapsulation|Decapsulation|
|:--|:--|:--|
Kyber512|23230|17594|
Kyber512 w/MAC|29166|14089|
Kyber768|33055|22650|
Kyber768 w/MAC|41077|17747|
Kyber1024|43893|33827|
Kyber1024 w/MAC|77011|27857|

For some additional context

||IND-CPA Encryption|IND-CPA Decryption|IND-CPA ciphertext size|KMAC computation|
|:--|:--|:--|:--|:--|
|Kyber512|10165|825|8359|768|
|Kyber768|19279|1045|10619|1088|
|Kyber1024|24891|1338|13306|1568|


The decrease in decapsulation runtime is minimal, while the increase in encapsulation runtime is substantial. There are many optimizations:

1. KMAC is implemented using Shake128/Shake256, which is known to be computationally expensive (between 5 and 10 cycles per byte of input data). A MAC based on polynomial and binary finite field (such as what's used in AES-GCM or ChaCha20-Poly1305) should be more efficient.
1. Kyber's round 3 uses the variation of modular Fujisaki-Okamoto transformation that outputs the shared key $K = H(m, c)$ as the hash of both the plaintext and the ciphertext. In fact, it is $K = H(m, H(c))$, so the hash of the ciphertext is already computed. We might be able to compute the tag on the hash of the ciphertext (32 bytes) instead of the ciphertext (more than 1000 bytes) itself.

We considered some additional strategies, such as using "encrypt-and-mac" and "mac-then-encrypt". The logistics problem of "mac-then-encrypt" aside, both strategies have the problem that the tag is not computed on the ciphertext, so the malleability of many lattice-based ciphertext means that a decryption oracle will trivially decrypt a tempered ciphertext.

## Using a one-time MAC
When considering the OW-PCVA security game, the challenge encryption is performed by:

1. Uniformly sampling a random message $m^\ast \leftarrow\$ \mathcal{M}$
2. Deriving the pseudorandom seed and the MAC key $(r^\ast, k^\ast) \leftarrow G(m^\ast)$
3. Deterministically encrypt the message $c^\ast \leftarrow E(\texttt{pk}, m^\ast; r^\ast)$
4. Compute the tag $t^\ast \leftarrow \texttt{MAC}(k^\ast, c^\ast)$
5. The authenticated challenge ciphertext $(c^\ast, t^\ast)$ is given to the adversary

Notice that **the authenticated challenge ciphertext is the only message-tag pair under $k^\ast$ that the adversary can see**. There is no signing oracle for the adversary to query any other message-tag pairs. The adversary can attempt forgeries and send the attempt $(\hat{c}, \hat{t})$ to the ciphertext-validation oracle to check if they are a valid pair. Assuming the MAC to secure, then the adversary will have no better way than to guess the tag for any ciphertext. In addition, in the real world, an honest peer can easily implement rate limiting (like blocking the adversary if there are more than 1000 invalid ciphertexts), which can thwart any brute-force forgery attack.

This means that we can actually use an one-time MAC, such as Poly1305 (by itself, instead of Poly1305-AES, which is a many-time MAC using a nonce). Poly1305 is a desirable candidate because of its performance advantage over GCM or KMAC.
