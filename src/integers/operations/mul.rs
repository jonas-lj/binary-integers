use crate::integers::natural_numbers::NaturalNumber;
use crate::integers::Integer;
use crate::integers::Integer::{NonZero, Zero};
use num_traits::One;
use std::mem;
use std::ops::{Mul, MulAssign};

#[allow(clippy::suspicious_op_assign_impl)]
impl MulAssign<&Integer> for Integer {
    fn mul_assign(&mut self, rhs: &Integer) {
        match (&mut *self, rhs) {
            (_, Zero) | (Zero, _) => {}
            (&mut NonZero(ref mut x, ref mut sign_x), NonZero(y, sign_y)) => {
                x.mul_assign(y);
                let _ = mem::replace(sign_x, *sign_x ^ *sign_y);
            }
        }
    }
}

impl Mul<Integer> for Integer {
    type Output = Integer;

    fn mul(self, rhs: Integer) -> Self::Output {
        let mut result = self;
        result *= &rhs;
        result
    }
}

impl One for Integer {
    fn one() -> Self {
        NonZero(NaturalNumber::one(), false)
    }
}
