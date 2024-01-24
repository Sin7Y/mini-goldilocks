use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num::BigUint;

pub trait Square {
    fn square(&self) -> Self;
}

pub trait Field:
    'static
    + Copy
    + Eq
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sum
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Square
    + Default
    + Display
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const NEG_ONE: Self;

    /// The 2-adicity of this field's multiplicative group.
    const TWO_ADICITY: usize;

    /// The field's characteristic and it's 2-adicity.
    /// Set to `None` when the characteristic doesn't fit in a u64.
    const CHARACTERISTIC_TWO_ADICITY: usize;

    /// Generator of the entire multiplicative group, i.e. all non-zero
    /// elements.
    const MULTIPLICATIVE_GROUP_GENERATOR: Self;
    /// Generator of a multiplicative subgroup of order `2^TWO_ADICITY`.
    const POWER_OF_TWO_GENERATOR: Self;

    /// The bit length of the field order.
    const BITS: usize;

    fn order() -> BigUint;
    fn characteristic() -> BigUint;

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    #[inline]
    fn is_nonzero(&self) -> bool {
        *self != Self::ZERO
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == Self::ONE
    }

    #[inline]
    fn double(&self) -> Self {
        *self + *self
    }

    #[inline]
    fn cube(&self) -> Self {
        self.square() * *self
    }

    fn triple(&self) -> Self {
        *self * (Self::ONE + Self::TWO)
    }

    fn primitive_root_of_unity(n_log: usize) -> Self {
        assert!(n_log <= Self::TWO_ADICITY);
        let base = Self::POWER_OF_TWO_GENERATOR;
        base.exp_power_of_2(Self::TWO_ADICITY - n_log)
    }

    /// Returns `n`. Assumes that `n` is already in canonical form, i.e. `n <
    /// Self::order()`.
    // TODO: Should probably be unsafe.
    fn from_canonical_u64(n: u64) -> Self;

    fn exp_power_of_2(&self, power_log: usize) -> Self {
        let mut res = *self;
        for _ in 0..power_log {
            res = res.square();
        }
        res
    }

    /// Representative `g` of the coset used in FRI, so that LDEs in FRI are
    /// done over `gH`.
    fn coset_shift() -> Self {
        Self::MULTIPLICATIVE_GROUP_GENERATOR
    }

    /// Equivalent to *self + x * y, but may be cheaper.
    #[inline]
    fn multiply_accumulate(&self, x: Self, y: Self) -> Self {
        // Default implementation.
        *self + x * y
    }
}

pub trait Field64: Field {
    const ORDER: u64;

    fn to_canonical_u64(&self) -> u64;

    /// Returns `x % Self::CHARACTERISTIC`.
    // TODO: Move to `Field`.
    fn from_noncanonical_u64(n: u64) -> Self;

    #[inline]
    // TODO: Move to `Field`.
    fn add_one(&self) -> Self {
        unsafe { self.add_canonical_u64(1) }
    }

    #[inline]
    // TODO: Move to `Field`.
    fn sub_one(&self) -> Self {
        unsafe { self.sub_canonical_u64(1) }
    }

    /// # Safety
    /// Equivalent to *self + Self::from_canonical_u64(rhs), but may be cheaper.
    /// The caller must ensure that 0 <= rhs < Self::ORDER. The function may
    /// return incorrect results if this precondition is not met. It is
    /// marked unsafe for this reason.
    // TODO: Move to `Field`.
    #[inline]
    unsafe fn add_canonical_u64(&self, rhs: u64) -> Self {
        // Default implementation.
        *self + Self::from_canonical_u64(rhs)
    }

    /// # Safety
    /// Equivalent to *self - Self::from_canonical_u64(rhs), but may be cheaper.
    /// The caller must ensure that 0 <= rhs < Self::ORDER. The function may
    /// return incorrect results if this precondition is not met. It is
    /// marked unsafe for this reason.
    // TODO: Move to `Field`.
    #[inline]
    unsafe fn sub_canonical_u64(&self, rhs: u64) -> Self {
        // Default implementation.
        *self - Self::from_canonical_u64(rhs)
    }
}
