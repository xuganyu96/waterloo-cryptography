While working on a project for benchmarking TLS handshake performance using `rustls`, I rant into a problem where the client will fail to validate the X.509 Certificate. More precisely, somewhere during the handshake, a function call in the client returned an `Err` variant of a result. The encapsulated error type is `InvalidCertificate::NotValidForName`. Subsequently the server will also receive an alert (a TLS message type) with the error message `BadCertificate`.

There isn't much helpful resources on the Internet about this error message. After some quick digging, the only clue I have is the "limitation" section on [`webpki`](https://github.com/rustls/webpki)'s GitHub README:

> Notably it does not offer support for self-signed Certificate

In addition, I feel confident that the server-side setup (generating CA certificates, generating server key pairs, and signing the server key with the generated CA certificates) is correct because both `curl` and `openssl` can finish the handshake process and verify the certificates. Part of the output from `openssl s_client -connect <host>:<port> -tls1_3 -verifyCAfile ca-cert.pem`:

```
---
No client certificate CA names sent
Peer signing digest: SHA512
Peer signature type: RSA-PSS
Server Temp Key: X25519, 253 bits
---
SSL handshake has read 2322 bytes and written 309 bytes
Verification: OK
---
New, TLSv1.3, Cipher is TLS_AES_256_GCM_SHA384
Server public key is 4096 bit
This TLS version forbids renegotiation.
Compression: NONE
Expansion: NONE
No ALPN negotiated
Early data was not sent
Verify return code: 0 (ok)
---
```
