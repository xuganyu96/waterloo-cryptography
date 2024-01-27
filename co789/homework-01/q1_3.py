from sympy import Rational

if __name__ == "__main__":
    q = 3329
    n = 256
    prod = 1
    for i in range(n):
        prod *= 1 - Rational(q) ** (i - n)
    print(f"Prob is {prod.evalf()}")
