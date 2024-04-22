# Montgomery Multiplication

## Montgomery form
Let $n$ be the modulus and $r$ be some integer such that $r \geq n$ and $\gcd(r, n) = 1$, then using the extended Euclid algorithm we can find integer $r^{-1}$ and $n^{-1}$ such that:

$$
rr^{-1} + nn^{-1} = 1
$$

## Montgomery reduction
Suppose $x, y$ are two integers, and $\hat{x}, \hat{y}$ are their Montgomery representation respectively:

$$
\begin{aligned}
    \hat{x} &= x \cdot r \mod n \\
    \hat{y} &= y \cdot r \mod n
\end{aligned}
$$

Then the Montgomery form of the product $xy$ is can be expressed using the Montgomery forms of $x, y$ individually:

$$
\hat{xy} = \hat{x}\hat{y}r^{-1} \mod n
$$

In other words, to compute the product in Montgomery form, we need a way to "multiply by $r^{-1}$ modulus $n$" efficiently.

Suppose $x$ is some integer such that $0 \leq x < n^2$ (e.g. x is the product of two integers in Montgomery space), then observe the following:

$$
\begin{aligned}
    xr^{-1} &\equiv \frac{xrr^{-1}}{r} \mod n \\
\end{aligned}
$$

Where the fraction indicates integer division. While generic integer division is expensive and variable-time, for appropriately chosen values of $r$ such as some power of 2 (e.g. $r = 2^{32}$), we can compute the quotient using bit-shifting, which is fast and constant-time with respect to the value being shifted.

We continue the computation by observing that $rr^{-1} + nn^{-1} = 1$:

$$
\begin{aligned}
    \frac{xrr^{-1}}{r} &\equiv \frac{x(1 - nn^{-1})}{r} \mod n \\
    &\equiv \frac{x - xnn^{-1}}{r} \mod n \\
\end{aligned}
$$

Notice that for any integer $l$, adding $ln$ does not change the modulus relationship. Therefore for any $l$ we have:

$$
\begin{aligned}
    \frac{x - xnn^{-1}}{r}
    &\equiv \frac{x - xnn^{-1}}{r} + nl \mod n \\
    &\equiv \frac{x - xnn^{-1} + rnl}{r} \mod n \\
    &\equiv \frac{x - n(xn^{-1} - rl)}{r} \mod n
\end{aligned}
$$

Since $l$ can be any integer, we can pick $l$ such that $0 \leq xn^{-1} - rl < r$. Let $q$ denote such value $q \leftarrow xn^{-1} \mod r$. Here modulus by $r$ can be efficiently computed if $r$ is a power of 2 using bitmasking:

$$
xr^{-1} \equiv \frac{x - qn}{r} \mod n
$$

We assumed $0 \leq x < n^{2}$, picked $l$ such that $0 \leq q < r$, and since $r$ is chosen such that $r > n$, we have:

$$
-n < \frac{x - qn}{r} < n
$$

which means that this value is at most one addition away from being in the correct range $[0, n)$.

To summarize:

```python
def montgomery_reduce(x, r, n):
    # r_inv and n_inv can both be pre-computed
    r_inv = ...
    n_inv = ...

    # modulus a power of 2 can be efficiently implemented using a bitmask
    q = mod2k(x * n_inv, r)
    # TODO: need to make this constant-time
    if x - qn < 0:
        q -= r

    # this needs to be a constant time ternary expression
    reduced = x - q * n if x >= q*n else x - (q-1) * n

    return reduced >> log2(r)
```