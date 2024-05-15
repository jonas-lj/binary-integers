use crate::integers::Integer;
use crate::integers::Integer::{NonZero, Zero};
use std::mem;
use std::ops::Neg;

impl Neg for Integer {
    type Output = Integer;

    fn neg(mut self) -> Self::Output {
        match self {
            Zero => {}
            NonZero(_, ref mut sign) => {
                let _ = mem::replace(sign, !*sign);
            }
        }
        self
    }
}

impl Neg for &Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        match self {
            Zero => Zero,
            NonZero(x, sign) => NonZero(x.clone(), !sign),
        }
    }
}
