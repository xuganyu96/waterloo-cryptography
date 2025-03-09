# What's next

## post quantum TLS/DTLS
- Interesting method of TLS Certificates using [Merkle Tree](https://datatracker.ietf.org/doc/draft-davidben-tls-merkle-tree-certs/04/)
- More work on KEMTLS
    - Thom Wiggers' [PhD thesis](https://thomwiggers.nl/publication/thesis/thesis.pdf) claimed that "the public keys of Classic McEliece [7] are too large to use in TLS 1.3." I think he is referring to the limit of message size in the record layer, but the **Certificate** message can be split into two records. Public key size is still a concern, but client need to do less computation and the returned ciphertext is smaller
    - Pre-distributed key might be a good option for embedded systems. The IoT device (tiny client) has a copy of the server's public key baked into the firmware. Tiny client then performs KEMTLS-PDK handshake with the server. This scenario was not covered in the PhD thesis mentioned above. One problem is forward secrecy: if the server's secret key was stolen, then the client will need to fallback onto regular TLS/KEMTLS, then re-distribute the long term public key.
    - Classic McEliece also has an interesting property where the encapsulation can work with partial public key, so the client can start encapsulation while still receiving public key. This also means that even if the public key doesn't fit into an IoT device's memory (`348864` pk takes 260KB, `460896` takes 524 KB, `6688128` pk takes ~1MB), the devcie can still run the encapsulation
- Will large key size be a problem in DTLS handshake? Probably not because the sequence number is simply moved from 

## Code-based cryptography

## Implementing post-quantum signature and key exchange in Bluetooth LE
- [This](https://github.com/bluekitchen/btstack) is Pi Pico W's Bluetooth stack
- [Micro:bit v2](https://www.nordicsemi.com/Products/Development-software/s113) also has bluetooth. I need to find its source code

## Make Raspberry Pi Pico into a YubiKey?

## Re-visit cryptography fundamentals
- What is PRF, PRP? 
- How to model hash function and/or MAC as PRF?
- Merkle-Damgaard construction, why it is insecure if used directly has a MAC, why is HMAC secure?