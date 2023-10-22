import hashlib

SPECIAL_CHARS = "!?*$#"
PWD_LEN = 12
SALT = "89535971"
HASH = "9edabf325bc63599725e1aa6a01a9b1a0bc09ff264bdbe7debb35f73a352a2cc"

def password_gen(
    special_chars: str,
    password_len: int,
    words: list[str],
):
    for word in words:
        word = word.capitalize()
        ndigits = password_len - 1 - len(word)
        for num in range(10 ** ndigits):
            for special_char in special_chars:
                num = str(num).zfill(ndigits)
                password = word + num + special_char
                yield password

if __name__ == "__main__":
    with open("inputs/a3q2word_list.txt") as f:
        words = f.read().splitlines()

    nhashes = 0
    for password in password_gen(SPECIAL_CHARS, PWD_LEN, words):
        password = SALT + password
        hash = hashlib.sha256(password.encode()).hexdigest()
        nhashes += 1
        if hash == HASH:
            print(password)
            print(nhashes)