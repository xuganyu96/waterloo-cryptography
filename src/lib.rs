/// return (x - y) with modulo; the value should in 0..modulo
pub fn modulo_sub(x: i64, y: i64, modulo: i64) -> i64 {
    if x >= y {
        return (x - y) % modulo;
    }
    let multiplier = (y + modulo - x) / modulo;
    return modulo_sub(x + modulo * multiplier, y, modulo);
}

/// https://github.com/TheAlgorithms/Rust/blob/master/src/math/extended_euclidean_algorithm.rs
fn update_step(a: &mut i64, old_a: &mut i64, quotient: i64) {
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}

/// https://github.com/TheAlgorithms/Rust/blob/master/src/math/extended_euclidean_algorithm.rs
pub fn extended_euclidean_algorithm(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coeff_s) = (1, 0);
    let (mut old_t, mut coeff_t) = (0, 1);

    while rem != 0 {
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coeff_s, &mut old_s, quotient);
        update_step(&mut coeff_t, &mut old_t, quotient);
    }

    (old_r, old_s, old_t)
}

/// Find inverse of x under the input modulo, assuming that x and modulo are relatively prime
///
/// Use the extended Euclid algorithm to solve for the Bezout coefficients:
/// x * u + modulo * v = gcd(x, modulo) = 1
/// Pseudocode copied from:
/// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn modulo_invert(x: i64, modulo: i64) -> i64 {
    let (_gcd, s, _t) = extended_euclidean_algorithm(x, modulo);
    return modulo_sub(s, 0, modulo);
}

/// Represent a single congruence in the form of:
/// x = r (mod q)
#[derive(Debug, Clone)]
struct Congruence {
    remainder: i64,
    modulo: i64,
}

impl Congruence {
    fn new(remainder: i64, modulo: i64) -> Self {
        return Self {
            remainder: remainder % modulo,
            modulo,
        };
    }
}

#[derive(Debug)]
pub struct CongruenceSolver {
    /// The set of congruences that this solver currently solves
    congruences: Vec<Congruence>,

    /// The modulo up to which the solution is unique. In the context of Chinese remainder theorem,
    /// it is the product of all modulos in all congruences
    modulo: i64,

    /// The solution to the set of congruences, unique up to self.modulo.
    /// At initialization when there is no equations to solve, there is no solution, hence the
    /// Option
    sol: Option<i64>,
}

impl CongruenceSolver {
    pub fn new() -> Self {
        return Self {
            congruences: vec![],
            sol: None,
            modulo: 1,
        };
    }

    /// Return True iff the current solution satisfies all existing congruences; empty solution is
    /// valid
    pub fn validate(&self) -> bool {
        if self.congruences.is_empty() {
            return true;
        }
        return self.congruences.iter().all(|cong| {
            let sol = self.sol.unwrap();
            return sol % cong.modulo == cong.remainder;
        });
    }

    /// Return the solution and the corresponding modulo up to which this solution is unique
    pub fn get_sol(&self) -> (Option<i64>, i64) {
        return (self.sol, self.modulo);
    }

    pub fn add_congruence(&mut self, remainder: i64, modulo: i64) {
        let congruence = Congruence::new(remainder, modulo);
        self.update_self(congruence);
    }

    /// Update self such that self solves the new set of congruences that is the union of the
    /// existing set of congruences and the new congruence
    fn update_self(&mut self, congruence: Congruence) {
        if self.sol.is_none() {
            self.sol = Some(congruence.remainder);
            self.modulo = congruence.modulo;
            self.congruences.push(congruence);
        } else {
            let mut sol = self.sol.unwrap();
            // new_sol = self.sol + self.modulo * some_integer, where we need to find the
            // appropriate integer
            // self.sol + self.modulo * some_integer = new_remainder (mod new_modulo)
            // self.modulo * some_integer = new_reminader - self.sol (mod new_modulo)
            // some_integer = (self.modulo * -1) * (new_remainder - self.sol) (mod new_modulo)

            let diff = modulo_sub(congruence.remainder, sol, congruence.modulo);
            let inverse = modulo_invert(self.modulo, congruence.modulo);
            sol = sol + self.modulo * diff * inverse;
            self.modulo = self.modulo * congruence.modulo;
            self.sol = Some(sol % self.modulo);
            self.congruences.push(congruence);
        }
    }
}
