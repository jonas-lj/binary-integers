use crate::integers::Integer;
use crate::integers::Integer::{NonZero, Zero};
use crate::TwosType;
use std::ops::{ShlAssign, ShrAssign};

impl ShrAssign<TwosType> for Integer {
    fn shr_assign(&mut self, rhs: TwosType) {
        match self {
            Zero => {}
            NonZero(ref mut x, _) => x.shr_assign(rhs),
        }
    }
}

impl ShlAssign<TwosType> for Integer {
    fn shl_assign(&mut self, rhs: TwosType) {
        match self {
            Zero => {}
            NonZero(x, _) => x.shl_assign(rhs),
        }
    }
}
