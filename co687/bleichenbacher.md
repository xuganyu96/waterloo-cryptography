$n$ has 15360 bits, hash name has 128 bits (16 bytes), hash is 512 bits (SHA512). The improperly padded message $M$ then has the form below:

```
M = 00 80 [y bytes of 00] [x bytes of AA] FF NAME HASH GAR
```

For clarity we denote:

- `00 80 [00 .. 00] [AA .. AA]` is called the "header" $H$. We don't know yet how many bytes of 00's or AA's we want to put into the header, so just denote them by x (bytes of AA) and y (bytes of 00). The header has $2 + x + y$ bytes, which is $16 + 8x + 8y$ bits
- `FF [hash name] [hash]` is called the "data" $D$. The data has $8 + 128 + 512 = 648$ bits
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

There are some additional constraints on $x, y$ to make sure that the $\ldots$ terms in $(2^{2193} + z)^7$ actually falls in to the garbage, but after some trials and errors I think $x = 64, y = 128$ should work:

1. Sample $m$ and compute the data sector $D$
2. Check if $N = \frac{2}{3}(2^{8x}-1)\times 2^{648} + D$ is divisible by 7. If not, repeat step 1
1. Compute $z = \frac{N}{7} \times 2^{1538 - 8(x+y)}$
1. $\sigma = 2^{2193} + z$ is the forgery

# Example
```python
def forge(x, y):
    header_str = "0080" + "00" * x + "AA" * y
    header_val = eval("0x" + header_str)

    data_str = "FF" + "00" * (16 + 512 // 8 - 1) + "00"
    data_val = eval("0x" + data_str)
    N = 2 * (2 ** (8 * x) - 1) * (2 ** 648) // 3 + data_val
    # If the initial data guess is no good, then update the data guess
    data_val += (7 - N % 7)
    data_str = f"{data_val:X}"
    N = 2 * (2 ** (8 * x) - 1) * (2 ** 648) // 3 + data_val
    assert N % 7 == 0
    z = N // 7 * (2 ** (1538 - 8 * (x + y)))
    sigma = z + 2 ** 2193  # this is the forged signature

    # verify
    m_hat = sigma ** 7
    m_hat_hex = f"00{m_hat:X}"
    print("data:\n\t" + data_str)
    print(
        "recovered data:\n\t" 
        + m_hat_hex[len(header_str):len(header_str) + len(data_str)]
    )
```
