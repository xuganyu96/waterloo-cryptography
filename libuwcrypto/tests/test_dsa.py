import random
import unittest
from libuwcrypto.dsa import Params, KeyPair, verify_hashed, sign_hashed, forge_unbounded


class TestDSA(unittest.TestCase):
    """test the correctness of DSA and the forgery"""

    def test_gen2048(self):
        _ = Params.gen2048()

    def test_correctness(self):
        params = Params.gen2048()
        keypair = KeyPair.keygen(params)
        message = random.randint(0, params.q)
        sigma = sign_hashed(keypair, message)
        self.assertTrue(verify_hashed(keypair.pk, message, sigma, True))

    def test_unbounded_forgery(self):
        params = Params.gen2048()
        keypair = KeyPair.keygen(params)
        message = random.randint(0, params.q)
        sigma = forge_unbounded(message, keypair.pk, params)
        assert verify_hashed(keypair.pk, message, sigma, False)
