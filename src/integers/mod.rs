use std::cmp::Ordering;
use std::fmt::Debug;

use natural_numbers::NaturalNumber;
use num_bigint::{BigInt, BigUint, Sign, ToBigInt};
use num_traits::Zero;

use crate::integers::Integer::*;
use crate::{IntegerTraits, TwosType};

pub mod natural_numbers;
mod operations;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default)]
pub enum Integer {
    #[default]
    Zero,
    NonZero(NaturalNumber, bool),
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        BigInt::from(self) == BigInt::from(other)
    }
}

impl Eq for Integer {}

impl Integer {
    pub fn test_bit(&self, index: TwosType) -> bool {
        match self {
            Zero => false,
            NonZero(x, _) => x.test_bit(index),
        }
    }

    #[inline]
    pub fn bits(&self) -> u64 {
        match self {
            Zero => 0,
            NonZero(x, _) => x.bits(),
        }
    }

    fn reduce(&mut self) {
        match self {
            Zero => {}
            NonZero(x, _) => {
                x.reduce();
                if x.is_zero() {
                    *self = Zero;
                }
            }
        }
    }
}

impl IntegerTraits for Integer {
    /// Return the number of trailing zeros in the binary representation of this number.
    #[inline]
    fn trailing_zeros(&self) -> TwosType {
        match self {
            Zero => 0,
            NonZero(x, _) => x.trailing_zeros(),
        }
    }

    /// Divide this number with the largest power of 2 that divides it and return the exponent 2.
    #[inline]
    fn oddify(&mut self) -> TwosType {
        match self {
            Zero => 0,
            NonZero(ref mut x, _) => x.oddify(),
        }
    }

    #[inline]
    fn is_negative(&self) -> bool {
        matches!(self, NonZero(_, true))
    }

    #[inline]
    fn is_odd(&self) -> bool {
        match self {
            Zero => false,
            NonZero(x, _) => x.is_odd(),
        }
    }

    fn cmp_shifted(&self, exp: TwosType, other: &Self) -> Ordering {
        match (self.bits() + exp as u64).cmp(&other.bits()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match (self, other) {
                (Zero, Zero) => Ordering::Equal,
                (Zero, NonZero(_, false)) => Ordering::Less,
                (Zero, NonZero(_, true)) => Ordering::Greater,
                (NonZero(_, false), Zero) => Ordering::Greater,
                (NonZero(_, true), Zero) => Ordering::Less,
                (NonZero(_, true), NonZero(_, false)) => Ordering::Less,
                (NonZero(_, false), NonZero(_, true)) => Ordering::Greater,
                (NonZero(x, false), NonZero(y, false)) => x.cmp_shifted(exp, y),
                (NonZero(x, true), NonZero(y, true)) => x.cmp_shifted(exp, y).reverse(),
            },
        }
    }
}

impl<T: Into<BigInt>> From<T> for Integer {
    fn from(value: T) -> Self {
        let value = value.into();
        match value.sign() {
            Sign::Minus => NonZero(NaturalNumber::from(value.magnitude().clone()), true),
            Sign::NoSign => Zero,
            Sign::Plus => NonZero(NaturalNumber::from(value.magnitude().clone()), false),
        }
    }
}

impl From<&Integer> for BigInt {
    fn from(value: &Integer) -> Self {
        match value {
            Zero => BigInt::zero(),
            NonZero(x, false) => BigUint::from(x).to_bigint().unwrap(),
            NonZero(x, true) => -BigUint::from(x).to_bigint().unwrap(),
        }
    }
}
