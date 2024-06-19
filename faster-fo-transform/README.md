# Faster Fujisaki-Okamoto transformation
The manuscript can be found on [Overleaf](https://www.overleaf.com/read/qgqctkbwyskm#ef31da).

The 1999 Fujisaki-Okamoto transformation (FO transform for short) and the 2017 modular transformation (modular FO transform for short) take a public-key encryption scheme that is OW-CPA (or IND-CPA) secure and uses **de-randomization** and **re-encryption** to add ciphertext integrity. However, public-key encryption is usually a computationally expensive routine, so running the encryption routine as a sub-routine of the decryption routine leaves much computational efficiency to be desired. As the authors of the modular FO transform stated in their paper:

> It is an interesting open problem to come up with alternative transformations that get rid of derandomization or that dispense with re-encryption (which preserving efficiency)

In this project, we propose to replace re-encryption with a message authentication code (MAC). Given an OW-CPA secure encryption scheme, we perform an alternative transformation:

1. In the encryption routine, we derive a MAC key from the plaintext message, then sign the unauthenticated ciphertext using the MAC key. The ciphertext-tag pair is returned as the authenticated ciphertext.
2. In the decryption routine, the unauthenticated ciphertext is first decrypted, and the decryption is used to re-derive the MAC key. We then use the re-derived MAC key to verify the tag against the ciphertext: if the tag is valid, return the decryption, otherwise reject the ciphertext as invalid.

It's easy to see that if both Alice and Bob are honest, then the transformed scheme is correct. If the MAC is an ideal PRF, then it should not help any passive adversary with recovering the message. Finally, if the ciphertext is tempered with, then the tag will change unpredictably without knowing the MAC Key, so assuming the MAC to be unforgeable, the adversary cannot produce new valid ciphertexts without recovering the MAC key first. The key security reduction is as follows:

> If the input public-key encryption scheme is OW-CPA secure, and the MAC is existentially unforgeable, then the transformed scheme is OW-PCVA secure.

From here, it is easy to use the results from the modular FO transform to construct IND-CCA secure key encapsulation mechanism (KEM).