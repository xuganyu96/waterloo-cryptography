# Revision logs
- [ ] How does Cramer-Shoup achieve IND-CCA2 security?
- [ ] https://dl.acm.org/doi/abs/10.1145/3460120.3484819
- [ ] write down the search to decision reduction

## 2024-08-19
- changed Gen to Keygen?
- line 8: not only computational inefficiencies but also side-channel vulnerabilities since now decryption also makes computation that takes the plaintext as input
- line 28: re-encryption is computationally expensive, and requires the decryption as inputs to the encryption routine, which risks opening unintended side channel (think clangover)
- line 39: a preshared symmetric key
- remove line 43
- remove subsubsection 2.1.1
- line 80 need to change the notation of advantage
- line 83 we didn't discuss ciphertext validation attacks?
- 96 session key should be capitalized
- 98: decapsulation routine doesn't always use explicit rejection
- line 117 discuss one-time security?
- section 2.4 should be in a "relevant work" since we don't directly build on top of FO transform anymore
- 171: this part needs some more elaboration. the probability of forging valid tags under an unknown key with no chosen message attack is not a standard security notion for a MAC, but it is easy to quantify. we can make a passing mention here then give concrete values in the experimental section where a concrete MAC is chosen
- need to fill out the experimental section better