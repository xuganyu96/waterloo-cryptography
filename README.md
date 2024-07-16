# What's next

## EtM is an IND-CCA2 secure if and only if PKE is OW-PCA secure
It turns out that the [original EtM construction is not IND-CCA2 secure for arbitrary PKE](./faster-fo-transform/etm-kyber-is-insecure). However, not all is lost. I think the IND-CCA2 security for the EtM KEM is equivalent to the OW-PCA security of the input PKE.

For every IND-CCA2 adversary against the KEM, we can construct an OW-PCA adversary against the underlying PKE. To service decapsulation query, the OW-PCA adversary can search through the hash oracle's tape and use the plaintext-checking oracle to find the corresponding decryption if there is one.

In the other direction, if there exists an efficient OW-PCA adversary against the underlying PKE, then we can build an efficient IND-CCA2 adversary against the KEM. To service a plaintext-checking query $(\tilde{m}, \tilde{c})$, the IND-CCA2 adversary uses the queried plaintext to sign the queried ciphertext $\tilde{t} = \texttt{MAC}(G(\tilde{m}), c)$, then queries the decapsulation oracle on the ciphertext-tag pair $(\tilde{c}, \tilde{t})$. If $\tilde{m} = \texttt{Dec}(\texttt{sk}, \tilde{c})$, then $\texttt{Decap}((\tilde{c}, \tilde{t}))$ should return $H(\tilde{m}, \tilde{c})$. The KEM adversary can compute $H(\tilde{m}, \tilde{c})$ on its own and verify that the return values match. On the other hand, if $\tilde{m} \neq \texttt{Dec}(\texttt{sk}, \tilde{c})$, then the return values will not match. The KEM adversary can thus simulate the plaintext-checking oracle for the OW-PCA adversary.

Unfortunately, Kyber is not OW-PCA secure, so EtM as it is should not be directly applied to Kyber. I am not sure if there are other post-quantum candidates that can start with a OW-PCA secure PKE.

## Timing variability in Kyber/ML-KEM
There is a [compiler-introduced timing variability in Kyber](https://groups.google.com/a/list.nist.gov/g/pqc-forum/c/hqbtIGFKIpU). Maybe I can try to replicate the result, with Rust. The [Clangover](https://github.com/antoonpurnal/clangover) repository referenced the following two side-channel vulnerability papers:

- [Generic Side-channel attacks on CCA-secure lattice-based PKE and KEM schemes](https://eprint.iacr.org/2019/948)
- [Curse of Re-encryption: A Generic Power/EM Analysis on Post-Quantum KEMs](https://eprint.iacr.org/2021/849)