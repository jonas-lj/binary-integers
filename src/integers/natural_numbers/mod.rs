use crate::{TwosType, TwosTypeSigned};
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

#[derive(Clone, Debug)]
pub struct NaturalNumber {
    twos: TwosTypeSigned,
    value: BigUint,
}

impl PartialEq for NaturalNumber {
    fn eq(&self, other: &Self) -> bool {
        BigUint::from(self) == BigUint::from(other)
    }
}

impl Eq for NaturalNumber {}

impl NaturalNumber {
    pub fn test_bit(&self, index: TwosType) -> bool {
        if index < self.twos as TwosType {
            return false;
        }
        let index = (index as TwosTypeSigned - self.twos) as u64;
        self.value.bit(index)
    }

    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    #[inline]
    pub(super) fn into_reduced(mut self) -> Self {
        self.reduce();
        self
    }

    /// Ensures that the odd part is odd (if the value is non-zero) and adjust the `twos` parameter.
    #[inline]
    pub(super) fn reduce(&mut self) {
        if self.twos < 0 {
            self.value >>= -self.twos as usize;
            self.twos = 0;
        }

        if self.value.is_zero() {
            self.twos = 0;
            return;
        }

        if self.value.is_odd() {
            return;
        }

        let zeros = self.value.trailing_zeros().unwrap();
        self.value >>= zeros;
        self.twos += zeros as TwosTypeSigned;
    }

    #[inline]
    pub(super) fn one() -> Self {
        NaturalNumber {
            twos: 0,
            value: BigUint::one(),
        }
    }

    #[inline]
    pub(crate) fn is_odd(&self) -> bool {
        if self.twos.is_positive() {
            return false;
        }
        self.test_bit(-self.twos as TwosType)
    }

    #[inline]
    pub(super) fn bits(&self) -> u64 {
        self.twos as u64 + self.value.bits()
    }

    /// True if self << shifts > other
    pub fn cmp_shifted(&self, shifts: TwosType, other: &Self) -> Ordering {
        match (self.bits() + shifts as u64).cmp(&other.bits()) {
            Less => Less,
            Greater => Greater,
            Equal => BigUint::from(self).shl(shifts).cmp(&BigUint::from(other)),
        }
    }

    #[inline]
    pub(crate) fn trailing_zeros(&self) -> TwosType {
        if self.twos.is_negative() {
            let mut i: TwosType = 0;
            while !self
                .value
                .bit((i as TwosTypeSigned - self.twos.abs()) as u64)
            {
                i += 1;
            }
            i as TwosType
        } else {
            self.twos as TwosType + self.value.trailing_zeros().unwrap_or(0) as TwosType
        }
    }

    /// Divide this number with the largest power of 2 that divides it and return the exponent 2.
    #[inline]
    pub(crate) fn oddify(&mut self) -> TwosType {
        if self.twos.is_negative() {
            let zeros = self.trailing_zeros();
            self.value >>= -self.twos + zeros as TwosTypeSigned;
            zeros
        } else {
            self.reduce();
            mem::replace(&mut self.twos, 0) as TwosType
        }
    }

    pub(crate) fn increment(&mut self) {
        if self.is_odd() {
            if self.twos.is_negative() {
                self.value.set_bit((-self.twos) as u64, true);
            }
        }

        if self.twos.is_negative() {
            self.reduce();
        }

        if self.is_odd() {
            self.value += BigUint::one();
            self.reduce();
        } else {
            self.value <<= self.twos;
            self.value.add_assign(BigUint::one());
            self.twos = 0;
        }
    }
}

impl From<BigUint> for NaturalNumber {
    fn from(value: BigUint) -> Self {
        NaturalNumber {
            twos: 0,
            value: value,
        }
        .into_reduced()
    }
}

impl From<&NaturalNumber> for BigUint {
    fn from(value: &NaturalNumber) -> Self {
        match value.twos.cmp(&0) {
            Less => (&value.value).shr(-value.twos as usize),
            Equal => value.value.clone(),
            Greater => (&value.value).shl(value.twos as usize),
        }
    }
}
