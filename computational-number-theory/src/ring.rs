//! The integer modulus ring
use std::error::Error;

use crate::Uint;

/// Various ways integer ring arithmetics can go wrong
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RingError {
    /// The modulus of a ring cannot be zero
    ModulusIsZero,
}

impl std::fmt::Display for RingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RingError {}

/// The integer ring is defined by the modulus, which cannot be zero
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Ring<const L: usize> {
    pub modulus: Uint<L>,
}

impl<const L: usize> Ring<L> {
    pub fn new(modulus: Uint<L>) -> Result<Self, RingError> {
        if modulus == Uint::<L>::ZERO {
            return Err(RingError::ModulusIsZero);
        }

        Ok(Self { modulus })
    }
}

/// An element of the modulus
pub struct RingElem<const L: usize> {
    pub val: Uint<L>,
    pub ring: Ring<L>,
}

impl<const L: usize> RingElem<L> {
    pub fn new(val: Uint<L>, ring: Ring<L>) -> Self {
        todo!();
    }

    pub fn add(&self, other: &Self) -> Self {
        todo!();
    }

    pub fn sub(&self, other: &Self) -> Self {
        todo!();
    }

    pub fn mul(&self, other: &Self) -> Self {
        todo!();
    }

    pub fn modexp(&self, exp: &Uint<L>) -> Self {
        todo!();
    }
}
