use crate::TwosType;
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, Zero};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::mem;
use std::ops::{AddAssign, Shl, Shr};

mod add;
mod mul;
mod order;
mod shifts;
mod sub;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NaturalNumber {
    twos: TwosType,
    odd_part: BigUint,
}

impl NaturalNumber {
    pub fn is_zero(&self) -> bool {
        self.twos == 0 && self.odd_part.is_zero()
    }

    #[inline]
    pub(super) fn into_reduced(mut self) -> Self {
        self.reduce();
        self
    }

    /// Ensures that the odd part is odd (if the value is non-zero) and adjust the `twos` parameter.
    #[inline]
    pub(super) fn reduce(&mut self) {
        if self.odd_part.is_zero() {
            self.twos = 0;
            return;
        }

        if self.odd_part.is_odd() {
            return;
        }
        let zeros = self.odd_part.trailing_zeros().unwrap();
        self.odd_part >>= zeros;
        self.twos += zeros as TwosType;
    }

    #[inline]
    pub(super) fn one() -> Self {
        NaturalNumber {
            twos: 0,
            odd_part: BigUint::one(),
        }
    }

    #[inline]
    pub(crate) fn is_odd(&self) -> bool {
        self.twos == 0
    }

    #[inline]
    pub(super) fn bits(&self) -> u64 {
        self.twos as u64 + self.odd_part.bits()
    }

    /// True if self << shifts > other
    pub fn cmp_shifted(&self, shifts: TwosType, other: &Self) -> Ordering {
        match (self.bits() + shifts as u64).cmp(&other.bits()) {
            Less => Less,
            Greater => Greater,
            Equal => match (self.twos + shifts).cmp(&other.twos) {
                Less | Equal => (&self.odd_part)
                    .shr(other.twos - self.twos - shifts)
                    .cmp(&other.odd_part),
                Greater => self
                    .odd_part
                    .cmp(&(&other.odd_part).shl(self.twos + shifts - other.twos)),
            },
        }
    }

    #[inline]
    pub(crate) fn trailing_zeros(&self) -> TwosType {
        self.twos
    }

    /// Divide this number with the largest power of 2 that divides it and return the exponent 2.
    #[inline]
    pub(crate) fn oddify(&mut self) -> TwosType {
        mem::replace(&mut self.twos, 0)
    }

    pub(crate) fn increment(&mut self) {
        if self.is_odd() {
            self.odd_part += BigUint::one();
            self.reduce();
        } else {
            self.odd_part <<= self.twos;
            self.odd_part.add_assign(BigUint::one());
            self.twos = 0;
        }
    }
}

impl From<BigUint> for NaturalNumber {
    fn from(value: BigUint) -> Self {
        NaturalNumber {
            twos: 0,
            odd_part: value,
        }
        .into_reduced()
    }
}

impl From<&NaturalNumber> for BigUint {
    fn from(value: &NaturalNumber) -> Self {
        (&value.odd_part).shl(&value.twos)
    }
}
