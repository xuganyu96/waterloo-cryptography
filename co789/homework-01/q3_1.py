import math

KYBER_Q = 3329

def centered_bin_pmf(val, n, p):
    if not (0 <= val + n * p <= n):
        return 0
    return (
        math.comb(n, int(val + n * p))
        * (p ** (val + n * p))
        * ((1-p) ** (n - val - n * p))
    )

def rho(val, mu, var):
    return math.exp(-(val - mu) ** 2 / (2 * var))

def dgaus(val, mu , var):
    return rho(val, mu, var) / sum(
        [rho(y, mu, var) for y in range(-KYBER_Q // 2, KYBER_Q // 2 + 1)]
    )

if __name__ == "__main__":
    n, p = 6, 0.5
    mu, var = 0, n * p * (1-p)
    dist = 0
    for val in range(
        math.ceil(mu - KYBER_Q / 2),
        math.ceil(mu + KYBER_Q / 2),
    ):
        lhs = centered_bin_pmf(val, n, p)
        rhs = dgaus(val, mu, var)
        dist += 0.5 * abs(lhs - rhs)
    print(dist)
