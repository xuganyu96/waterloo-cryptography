import unittest
from libuwcrypto.ec import EllipticCurve, Point

class TestEllipticCurve(unittest.TestCase):
    def test_ec(self):
        curve = EllipticCurve(13, 3, 8)
        coordinates = [
            (1, 5), (1, 8), (2, 3), (2, 10), (9, 6), (9, 7), (12, 2), (12, 11)
        ]
        points = [Point(x, y, curve) for x, y in coordinates]