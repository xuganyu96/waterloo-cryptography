- [x] Set up a simple TLS server that can perform handshake
- [ ] Establish a client connection to simple server
- [ ] Modify the TLS library to log handshake events

# Server-side setup
There are two requirements of a TLS server:

- A X.509 certificate, which for the purpose of benchmarking, will be self-signed
- A long-term TLS secret key

```bash
#!/bin/bash
#
# Generate CA's certificate and private key, then generate server's certificate and private key

# CA's Certificate and private key
openssl req -x509 \
    -newkey rsa:4096 \
    -days 365 \
    -keyout ca-key.pem \
    -out ca-cert.pem \
    -nodes \
    -subj "/C=CA/ST=Ontario/L=Waterloo/O=University of Waterloo/OU=Faculty of Engineering/CN=localhost"

# Server's private key and certificate request
openssl req -newkey rsa:4096 \
    -keyout server-key.pem \
    -out server-req.pem \
    -nodes \
    -subj "/C=CA/ST=Ontario/L=Waterloo/O=University of Waterloo/OU=Faculty of Engineering/CN=localhost"
# Use CA's private key to sign server's public key
openssl x509 -req \
    -in server-req.pem \
    -CA ca-cert.pem \
    -CAkey ca-key.pem \
    -CAcreateserial \
    -out server-cert.pem
# Verify Server's certificate using CA's certificate
openssl verify -CAfile ca-cert.pem server-cert.pem
```

We will pass the `server-cert.pem` and `server-key.pem` to a simple TLS server binary, which is scripted using [Rust](https://github.com/rustls/rustls/blob/main/examples/src/bin/simpleserver.rs). Run the server using the command:

```bash
cargo run --bin simpleserver server-cert.pem server-key.pem
```

Verify that the server can perform TLS handshake by using `curl` to send a HTTPS request. The HTTP request itself will fail since the TLS server does not handle HTTP requests, but we can pass the `-vv` flag in the `curl` command to display handshake information:

```bash
curl https://localhost:4443 \
    # pass in the custom CA certificate
    --cacert ca-cert.pem \
    -vv
```

The output is as follows:

```
*   Trying [::1]:4443...
* Connected to localhost (::1) port 4443
* ALPN: curl offers h2,http/1.1
* (304) (OUT), TLS handshake, Client hello (1):
*  CAfile: ca-cert.pem
*  CApath: none
* (304) (IN), TLS handshake, Server hello (2):
* (304) (IN), TLS handshake, Unknown (8):
* (304) (IN), TLS handshake, Certificate (11):
* (304) (IN), TLS handshake, CERT verify (15):
* (304) (IN), TLS handshake, Finished (20):
* (304) (OUT), TLS handshake, Finished (20):
* SSL connection using TLSv1.3 / AEAD-CHACHA20-POLY1305-SHA256
* ALPN: server did not agree on a protocol. Uses default.
* Server certificate:
*  subject: C=CA; ST=Ontario; L=Waterloo; O=University of Waterloo; OU=Faculty of Engineering; CN=localhost
*  start date: Feb  9 00:33:00 2024 GMT
*  expire date: Mar 10 00:33:00 2024 GMT
*  common name: localhost (matched)
*  issuer: C=CA; ST=Ontario; L=Waterloo; O=University of Waterloo; OU=Faculty of Engineering; CN=localhost
*  SSL certificate verify ok.
```

Another way to perform initiate the TLS handshake from the client side is by using `openssl s_client`:

```bash
openssl s_client -connect <hostname>:<port> -tls1_3 -verifyCAfile ca-cert.pem
```

# Client-side setup
Since we need to eventually replace the classic TLS cipher suites with post-quantum primitives, `curl` and `openssl` as ready-made solutions are insufficient. Instead, the `rustls` library will (eventually) be modified to allow the drop-in replacements.

For now though, it is sufficient to create a client-side setup that simply perform a handshake, send no application data, then hang up.

```
rustls's NotValidForName is equivalent to
webpki::Error::CertNotValidForName
```

The `webpki` crate [does not support self-signed certificate](https://github.com/rustls/webpki?tab=readme-ov-file#limitations). The author of `webpki` suggests using `x509-parser` and `rcgen`