//! Common modulo arithmetics and algorithms

/// At this moment a brute-force attempt to find all square roots of x within
/// the modulo
pub fn sqrt(x: i64, modulo: i64) -> Vec<i64> {
    return (0..modulo).filter_map(|elem| {
        if modulo_sub(elem * elem, x, modulo) == 0 {
            return Some(elem);
        }
        return None;
    })
    .collect();
}

/// subtract y from x within the input modulo
pub fn modulo_sub(x: i64, y: i64, modulo: i64) -> i64 {
    let x = x % modulo;
    let y = y % modulo;

    if x - y >= 0 {
        return x - y;
    }
    return x - y + modulo;
}

/// Returns (gcd, s, t) such that s*x + t*y = gcd is the Bezout identity
pub fn extended_gcd(x: i64, y: i64) -> (i64, i64, i64) {
    let (mut prev_r, mut r) = (x, y);
    let (mut prev_s, mut s) = (1, 0);
    let (mut prev_t, mut t) = (0, 1);

    while r != 0 {
        let q = prev_r / r;
        (prev_r, r) = (r, prev_r - q * r);
        (prev_s, s) = (s, prev_s - q * s);
        (prev_t, t) = (t, prev_t - q * t);
    }

    return (prev_r, prev_s, prev_t);
}

/// Attempt to find a multiplicative inverse of x (mod y). This is possible iff
/// x and y are relatively prime. If no multiplicative inverse if possible,
/// return None
pub fn modulo_invert(x: i64, y: i64) -> Option<i64> {
    let (gcd, s, _t) = extended_gcd(x, y);
    if gcd > 1 {
        return None;
    }
    return Some(s);
}

#[derive(Debug)]
pub struct CRT {
    /// The modulo up to wihch the solution is unique. In teh context of Chinese
    /// remainder theorem, it is the product of all modulos in all congruences
    modulo: i64,

    /// The solution to the set of congruences, unique up to self.modulo.
    sol: Option<i64>,
}

impl CRT {
    pub fn new() -> Self {
        return Self {
            sol: None,
            modulo: 1,
        };
    }

    /// Update the internal state to solve the union of the existing system and
    /// the input congruence. Return the solution after the update
    pub fn add_congruence(
        &mut self,
        remainder: i64,
        modulo: i64,
    ) -> Option<i64> {
        if self.sol.is_none() {
            self.sol = Some(remainder);
            self.modulo = modulo;
        } else {
            let mut sol = self.sol.unwrap();
            let diff = modulo_sub(remainder, sol, modulo);
            let inverse = modulo_invert(self.modulo, modulo).unwrap();
            sol = sol + self.modulo * diff * inverse;
            self.modulo = self.modulo * modulo;
            self.sol = Some(modulo_sub(sol, 0, self.modulo));
        }
        return self.sol;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulo_sub() {
        assert_eq!(modulo_sub(5, 7, 9), 7);
        assert_eq!(modulo_sub(7, 5, 9), 2);
        assert_eq!(modulo_sub(997, 999, 9), 7);
    }

    #[test]
    fn test_extended_gcd() {
        assert_eq!(extended_gcd(101, 13), (1, 4, -31));
        assert_eq!(extended_gcd(123, 19), (1, -2, 13));
        assert_eq!(extended_gcd(25, 36), (1, 13, -9));
        assert_eq!(extended_gcd(69, 54), (3, -7, 9));
        assert_eq!(extended_gcd(55, 79), (1, 23, -16));
        assert_eq!(extended_gcd(33, 44), (11, -1, 1));
        assert_eq!(extended_gcd(50, 70), (10, 3, -2));
    }

    #[test]
    fn test_modulo_invert() {
        assert_eq!(modulo_invert(5, 7).unwrap(), 3);
        assert!(modulo_invert(3, 6).is_none());
    }

    #[test]
    fn test_crt() {
        let mut solver = CRT::new();
        solver.add_congruence(3, 7);
        let sol = solver.add_congruence(4, 9);
        assert_eq!(sol, Some(31));

        let mut solver = CRT::new();
        solver.add_congruence(13, 71);
        let sol = solver.add_congruence(41, 97);
        assert_eq!(sol, Some(5764));

        let mut solver = CRT::new();
        solver.add_congruence(4, 7);
        solver.add_congruence(5, 8);
        let sol = solver.add_congruence(11, 15);
        assert_eq!(sol, Some(221));
    }
}
