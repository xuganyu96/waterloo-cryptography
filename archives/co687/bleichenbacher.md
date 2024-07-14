$n$ has 15360 bits, hash name has 128 bits (16 bytes), hash is 512 bits (SHA512). The improperly padded message $M$ then has the form below:

```
M = 00 80 [y bytes of 00] [x bytes of AA] FF [ASN] [DIG] [GAR]
```

Where 
- `ASN` is the hash name (128 bits)
- `DIG` is the digest of the message (512 bits)
- `GAR` is the garbage

For clarity we denote:

- `00 80 [00 .. 00] [AA .. AA]` is called the "header" $H$. We don't know yet how many bytes of 00's or AA's we want to put into the header, so just denote them by x (bytes of AA) and y (bytes of 00). The header has $2 + x + y$ bytes, which is $16 + 8x + 8y$ bits
- `FF [ASN] [DIG]` is called the "data". Denote the numerical value of data by $D$. The data has $8 + 128 + 512 = 648$ bits
- The rest is "gargage" $G$, which has $15360 - (16 + 8x + 8y) - 648 = 14696 - 8(x+y)$ bits

The numerical value of the header by itself `0x0080[00..00][AA..AA]` is

$$
H = 8 \times 2^{4 + 8x + 8y} + \frac{2}{3}(2^{8x} - 1)
$$

The numerical value of the entire padded message is thus

$$
\begin{aligned}
M &= H \times 2^{15360 - 16 - 8(x+y)} + D \times 2^{14696 - 8(x+y)} + G \\
&= (8 \times 2^{4 + 8x + 8y} + \frac{2}{3}(2^{8x} - 1)) \times 2^{15360 - 16 - 8(x+y)} + D \times 2^{14696 - 8(x+y)} + G \\
&= 2^{15351} + \frac{2}{3}(2^{8x} - 1)\times 2^{15344 - 8(x+y)} + D \times 2^{14696 - 8(x+y)} + G \\
&= 2^{15351} + \frac{2}{3}(2^{8x} - 1)\times 2^{648} \times 2^{14696 - 8(x+y)} + D \times 2^{14696 - 8(x+y)} + G \\
&= 2^{15351} + (\frac{2}{3}(2^{8x}-1)\times 2^{648} + D) \times 2^{14696 - 8(x+y)} + G
\end{aligned}
$$

We want $M$ to be some kind of 7-th power. Fortunately, $15351 = 2193 \times 7$, so we can guess that the 7-th root has the form $(2^{2193} + z)$

$$
\begin{aligned}
(2^{2193} + z)^7 &= 2^{15351} + 7z\cdot 2^{2193 \times 6} + \ldots \\
&= 2^{15351} + 7z\cdot 2^{13158} + \ldots \\
\end{aligned}
$$

This means that somehow we need to pick x, y such that

$$
7z\cdot 2^{13158} = (\frac{2}{3}(2^{8x}-1)\times 2^{648} + D) \times 2^{14696 - 8(x+y)}
$$

which is equivalent to

$$
7z = (\frac{2}{3}(2^{8x}-1)\times 2^{648} + D) \times 2^{1538 - 8(x+y)}
$$

This means that $(\frac{2}{3}(2^{8x}-1)\times 2^{648} + D)$ must be divisible by 7.

There are some additional constraints on $x, y$ to make sure that the $\ldots$ terms in $(2^{2193} + z)^7$ actually falls in to the garbage, but after some trials and errors I think $x = 32, y = 160$ should work:

1. Sample $m$ and compute the data sector $D$
2. Check if $N = \frac{2}{3}(2^{8x}-1)\times 2^{648} + D$ is divisible by 7. If not, repeat step 1
1. Compute $z = \frac{N}{7} \times 2^{1538 - 8(x+y)}$
1. $\sigma = 2^{2193} + z$ is the forgery

# Example
```python
MODULUS_BITS = 15360
MODULUS_BYTES = MODULUS_BITS // 8


def forge(pad_aa_bytes, pad_00_bytes):
    asn_str, digest_str = "00" * 16, "00" * 64
    data_str = "FF" + asn_str + digest_str
    data_val = eval("0x" + data_str)
    N = 2 * (2 ** (8 * pad_aa_bytes) - 1) * (2**648) // 3 + data_val
    # Ensure divisibility by 7
    data_val += 7 - N % 7
    data_str = f"{data_val:X}"
    asn_str, digest_str = data_str[4 : 4 + 16 * 2], data_str[-64 * 2 :]
    N = 2 * (2 ** (8 * pad_aa_bytes) - 1) * (2**648) // 3 + data_val
    assert N % 7 == 0
    z = N // 7 * (2 ** (1538 - 8 * (pad_aa_bytes + pad_00_bytes)))
    sigma = z + 2**2193  # this is the forged signature
    return asn_str, digest_str, sigma


def verify(sigma, e, verbose: bool = False):
    """Verify the signature"""
    padded_m_hat = pow(sigma, e)
    padded_m_hex = f"{padded_m_hat:X}"
    if len(padded_m_hex) < MODULUS_BYTES * 2:
        padded_m_hex = "0" * (2 * MODULUS_BYTES - len(padded_m_hex)) + padded_m_hex

    if padded_m_hex[:4] != "0080":
        raise ValueError("Header bits did not start with 0080")

    # find various segment start
    pad_00_start = 4
    pad_aa_start = padded_m_hex.find("AA")
    if pad_aa_start == -1:
        raise ValueError("AA padding not found")
    data_start = padded_m_hex.find("FF")
    if data_start == -1:
        raise ValueError("Data segment (FF...) not found")

    # Check 00 padding content and length
    for hex in padded_m_hex[pad_00_start:pad_aa_start]:
        if hex != "0":
            raise ValueError("Invalid 00 padding")
    if pad_aa_start - pad_00_start < 32 * 2:
        raise ValueError("00 padding too short")

    # Check AA padding content and length
    for hex in padded_m_hex[pad_aa_start:data_start]:
        if hex != "A":
            raise ValueError("Invalid AA padding")
    if data_start - pad_aa_start < 32 * 2:
        raise ValueError("AA padding too short")

    # Extract hash name and digest
    if len(padded_m_hex) - data_start < (1 + 16 + 64) * 2:
        raise ValueError("Data segment too short")
    asn = padded_m_hex[data_start + 2 : data_start + 2 + 16 * 2]
    digest = padded_m_hex[data_start + 2 + 16 * 2 : data_start + 2 + (16 + 64) * 2]

    if verbose:
        print(
            f"""Signature:
- 00 pad length: {(pad_aa_start - pad_00_start) // 2}
- AA pad length: {(data_start - pad_aa_start) // 2}
- ASN: {asn}
- Digest: {digest}
    """
        )
    return asn, digest


if __name__ == "__main__":
    for pad_00_len in range(112, 160 + 1):
        expected_asn, expected_digest, sigma = forge(32, pad_00_len)
        recovered_asn, recovered_digest = verify(sigma, 7, False)

        assert recovered_asn == expected_asn
        assert recovered_digest == expected_digest
```
