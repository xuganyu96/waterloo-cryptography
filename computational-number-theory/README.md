# Computational number theory
Rust implementation of algorithms described in [A Course in Computational Number Theory](https://link.springer.com/book/10.1007/978-3-662-02945-9) by Henri Cohen.

Next: simple modulus arithmetics
- [ ] Integer ring $Z_n$: addition, subtraction, multiplication
- [ ] `modexp`
- [ ] GCD and modulus inverse using (extended) Euclid's algorithm

# Problem 1: data structure layout
First we need to define our big integer as a struct. A good place to start would be an array of words, where each word is an unsigned 32-bit integers or other word size of your choosing.

After that, implement the following foundational operations:

1. Parsing from and encoding into hexadecimal strings
1. Overflowing addition and subtraction
1. Schoolbook multiplication 
1. Overflowing left and right bitshifting
1. Integer division and modulo