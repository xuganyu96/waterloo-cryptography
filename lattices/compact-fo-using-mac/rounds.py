"""Estimate the cost of various subroutines of Kyber, measured in the number of Keccak permutations
and the number of NTT operations (including NTT and invert_NTT)
"""
from enum import Enum

class SecurityLevels(Enum):
    Kyber512 = 1
    Kyber768 = 2
    Kyber1024 = 3

class KyberParameters:
    KYBER_Q = 3329
    KYBER_Q_BITS = 12
    KYBER_N = 256
    # Defaults to Kyber512 but will be modified
    KYBER_K = 2
    KYBER_ETA1 = 3
    KYBER_ETA2 = 2

    def __init__(self, level: SecurityLevels):
        if level == SecurityLevels.Kyber512:
            self.KYBER_K = 2
            self.KYBER_ETA1 = 3
            self.KYBER_ETA2 = 


