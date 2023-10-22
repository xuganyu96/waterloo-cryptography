"""A3Q1 code"""
import os
import getpass
import base64
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.hmac import HMAC
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives import padding

HKMAC_NITERS = 200000
# Params of AES128
KEYSIZE = 16
BLOCKSIZE = 16
BLOCKSIZE_BITS = 128
# Params of SHA3-256
TAGSIZE = 32


def prompt_password(confirm: bool = True, max_attempts: int = 10) -> bytes:
    """Repeatedly prompt the user for password until exceeding max attempts or
    receiving confirmed password.

    if confirm is False, the confirmation prompt will be skipped
    """
    for _ in range(max_attempts):
        pwd = getpass.getpass("Enter password:")
        if not confirm:
            return pwd.encode()
        again = getpass.getpass("Enter password again:")
        if again == pwd:
            return pwd.encode()
    raise TimeoutError("Too many failed attempts")


def derive_cipher_suite(confirm: bool = True, nonce: bytes | None = None):
    """Return the padder, the cipher, and the MAC from a single password

    If an input nonce is given, it will be used for the Block cipher CTR mode;
    otherwise a random nonce will be generated
    """
    hkdf = PBKDF2HMAC(
        algorithm=hashes.SHA3_256(),
        length=KEYSIZE,
        iterations=HKMAC_NITERS,
        salt=b"",
    )
    key = hkdf.derive(prompt_password(confirm))
    pad = padding.PKCS7(BLOCKSIZE_BITS)
    nonce = os.urandom(BLOCKSIZE) if nonce is None else nonce
    cipher = Cipher(algorithms.AES128(key), modes.CTR(nonce))
    mac = HMAC(key=key, algorithm=hashes.SHA3_256())

    return pad, cipher, nonce, mac


def base64_encode(bytes_: bytes) -> str:
    """Return the Base-64 encoding of the input bytes"""
    return base64.urlsafe_b64encode(bytes_).decode()


def base64_decode(str_: str) -> bytes:
    """Return the bytes that encode into the input base64 encoding"""
    return base64.urlsafe_b64decode(str_.encode())


def encrypt(plaintext: bytes):
    """Obtain cipher suite, compute and return ciphertext and tag"""
    pad, cipher, nonce, mac = derive_cipher_suite()
    encryptor = cipher.encryptor()
    padder = pad.padder()

    plaintext = padder.update(plaintext) + padder.finalize()
    ciphertext = encryptor.update(plaintext) + encryptor.finalize()
    mac.update(ciphertext)
    tag = mac.finalize()

    return nonce + ciphertext + tag


def decrypt(ciphertext: bytes) -> bytes:
    """Verify that the tag is authentic, then decrypt and recover the
    plaintext
    """
    nonce = ciphertext[:BLOCKSIZE]
    tag = ciphertext[-TAGSIZE:]
    ciphertext = ciphertext[BLOCKSIZE:-TAGSIZE]
    pad, cipher, nonce, mac = derive_cipher_suite(False, nonce)
    decryptor = cipher.decryptor()
    unpadder = pad.unpadder()

    mac.update(ciphertext)
    mac.verify(tag)

    plaintext = decryptor.update(ciphertext) + decryptor.finalize()
    plaintext = unpadder.update(plaintext) + unpadder.finalize()
    return plaintext


if __name__ == "__main__":
    msg = "Hello, world!".encode()
    msg_b64 = base64_encode(msg)
    print(f"plaintext bytes encode to {msg_b64}")

    ciphertext = encrypt(msg)
    print(f"ciphertext bytes encode to {base64_encode(ciphertext)}")
    decryption = decrypt(ciphertext)

    if decryption == msg:
        print("Decryption successful")
    else:
        print("Decryption failed")
