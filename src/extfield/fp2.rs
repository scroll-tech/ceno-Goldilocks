//! This module implements Goldilocks quadratic extension field mod x^2 - 7

use std::{
    mem::transmute,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use ff::Field;
use halo2curves::serde::SerdeObject;

use crate::{Goldilocks, GoldilocksExt2, SmallField};

use super::ExtensionField;

impl Mul<&Goldilocks> for &GoldilocksExt2 {
    type Output = GoldilocksExt2;

    #[inline]
    fn mul(self, rhs: &Goldilocks) -> Self::Output {
        let mut res = *self;
        res.mul_assign(rhs);
        res
    }
}

impl Mul<Goldilocks> for GoldilocksExt2 {
    type Output = GoldilocksExt2;

    #[inline]
    fn mul(self, rhs: Goldilocks) -> Self::Output {
        &self * &rhs
    }
}

impl<'a> Mul<&'a Goldilocks> for GoldilocksExt2 {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: &'a Goldilocks) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<&Goldilocks> for GoldilocksExt2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Goldilocks) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
    }
}

impl MulAssign<Goldilocks> for GoldilocksExt2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Goldilocks) {
        self.mul_assign(&rhs)
    }
}

impl Add<Goldilocks> for GoldilocksExt2 {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Goldilocks) -> Self::Output {
        self += &rhs;
        self
    }
}

impl<'a> Add<&'a Goldilocks> for GoldilocksExt2 {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: &'a Goldilocks) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Goldilocks> for GoldilocksExt2 {
    #[inline]
    fn add_assign(&mut self, rhs: Goldilocks) {
        *self += &rhs;
    }
}

impl<'a> AddAssign<&'a Goldilocks> for GoldilocksExt2 {
    #[inline]
    fn add_assign(&mut self, rhs: &'a Goldilocks) {
        self.0[0] += rhs;
    }
}

impl Sub<Goldilocks> for GoldilocksExt2 {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: Goldilocks) -> Self::Output {
        self -= &rhs;
        self
    }
}

impl<'a> Sub<&'a Goldilocks> for GoldilocksExt2 {
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: &'a Goldilocks) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<Goldilocks> for GoldilocksExt2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Goldilocks) {
        *self -= &rhs;
    }
}

impl<'a> SubAssign<&'a Goldilocks> for GoldilocksExt2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &'a Goldilocks) {
        self.0[0] -= rhs;
    }
}

impl ExtensionField for GoldilocksExt2 {
    /// Base field
    type BaseField = Goldilocks;

    /// Extension degree of the Field
    const DEGREE: usize = 2;

    /// Identifier string
    const NAME: &'static str = "GoldilocksExt2";

    /// Convert a byte string into a list of field elements
    fn bytes_to_field_elements(bytes: &[u8]) -> Vec<Self> {
        bytes
            .chunks(16)
            .map(Self::from_raw_bytes_unchecked)
            .collect::<Vec<_>>()
    }

    /// Convert a field elements to a u64 vector
    fn to_canonical_u64_vec(&self) -> Vec<u64> {
        self.0
            .iter()
            .map(|a| a.to_canonical_u64())
            .collect::<Vec<u64>>()
    }

    /// Unsafe function.
    /// Convert a field elements to a u64 vector. Do not normalize it.
    fn as_non_canonical_u64_slice(&self) -> &[u64] {
        unsafe { transmute::<&[Goldilocks], &[u64]>(self.0.as_slice()) }
    }

    /// Reference to limbs
    fn as_limbs(&self) -> &[Self::BaseField] {
        self.0.as_slice()
    }

    /// Convert limbs into self
    fn from_limbs(limbs: &[Self::BaseField]) -> Self {
        Self([limbs[0], limbs[1]])
    }

    /// Build a self from a base element; pad ext with 0s.
    fn from_base(b: &Self::BaseField) -> Self {
        Self([*b, Goldilocks::ZERO])
    }
}

impl GoldilocksExt2 {
    /// Convert self to limbs of Goldilocks elements
    fn to_limbs(&self) -> [<Self as ExtensionField>::BaseField; <Self as ExtensionField>::DEGREE] {
        self.0
    }
}
