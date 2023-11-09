"""Toy implementation of elliptic curve"""
from __future__ import annotations

class EllipticCurve:
    """(finite field) elliptic curve y^2 = x^3 + ax + b
    """
    def __init__(self, p, a, b):
        self.p = p
        self.a = a
        self.b = b
    
    def validate(self, x: int, y: int):
        """Return True iff and point is on the curve"""
        lhs = (y * y) % self.p
        rhs = ((x ** 3) + self.a * x + self.b) % self.p
        return lhs == rhs

    def solve_y(self, x: int):
        """Given a value of x, solve for y with brute-force. Always returns
        a pair of solution if it exists, otherwise return None
        """
        x = x % self.p
        for y in range(self.p):
            remainder = (y ** 2 - x ** 3 - self.a * x - self.b) % self.p
            if remainder == 0:
                return (y, self.p - y)
        return None
    
class Point:
    """A single point on the elliptic curve identified by the coordinate. The
    identity is identified by having no identity
    """
    def _linear_slope(self, other: Point):
        """assume that self and other are two points such that non of them is
        identity, they are not identical, and they are not negations of each
        other, then the slop of the line can be linearly computed
        """
        slope = pow(other.x - self.x, -1, self.curve.p)
        slope = (other.y - self.y) * slope
        slope = slope % self.curve.p
        return slope

    def _tagent_slope(self):
        """The slope of the tagent line cutting through this point"""
        slope = pow(2 * self.y, -1, self.curve.p)
        slope = (3 * self.x * self.x + self.curve.a) * slope
        return slope % self.curve.p

    @staticmethod
    def identity(curve: EllipticCurve):
        return Point(None, None, curve)
    
    def is_identity(self):
        return self.x is None
    
    def negation(self):
        if self.is_identity():
            return Point.identity(self.curve)
        return Point(self.x, -self.y + self.curve.p, self.curve)

    def __init__(self, x, y, curve: EllipticCurve):
        if x is not None and not curve.validate(x, y):
            raise ValueError("Point is not on the curve")
        self.x = x
        self.y = y
        self.curve = curve
    
    def __eq__(self, other):
        """Two points are equal if they are both identity or if they are
        coordinate-wise equal
        """
        if not isinstance(other, Point):
            raise TypeError("Equality with non-point not defined")
        return (
            (self.is_identity() and other.is_identity()) 
            or (self.x == other.x and self.y == other.y)
        )
    
    def __add__(self, other):
        if not isinstance(other, Point):
            raise TypeError("Addition with non-point not defined")
        
        # If one of the operand is identity then return the other
        if self.is_identity():
            return other
        if other.is_identity():
            return self
        
        # P + (-P) = 0
        if self == other.negation():
            return Point.identity(self.curve)
        
        # With P + Q where P, Q are not identity and not x-axis mirrors, there
        # is a common formula, although there is a slope term that is evaluated
        # differently depending on P == Q or P != Q
        slope = (
            self._tagent_slope() 
            if self == other else self._linear_slope(other)
        )
        x = ((slope * slope) - other.x - self.x) % self.curve.p
        y = (-slope * x - self.y + slope * self.x) % self.curve.p
        return Point(x, y, self.curve)
    
    def __mul__(self, other):
        if not isinstance(other, int):
            raise TypeError("Non-scaler multiplication not defined")
        prod = Point(self.x, self.y, self.curve)
        for _ in range(other-1):
            prod += Point(self.x, self.y, self.curve)
        return prod
        

    def __repr__(self):
        if self.is_identity():
            return "<Point 0>"
        return f"<Point ({self.x}, {self.y})>"

if __name__ == "__main__":
    curve = EllipticCurve(103, 3, 4)
    q = Point(2, 11, curve)
    p = Point(84, 68, curve)
    c = 3
    assert p == q * c

    t1 = 42
    sols = curve.solve_y(t1)
    r1q = Point(t1, sols[1], curve)
    r1p = r1q * c
    s1 = r1p.x; print(s1)
    r2 = (p * s1).x
    t2 = (q * r2).x; print(t2)