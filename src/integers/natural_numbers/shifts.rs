use std::ops::{ShlAssign, ShrAssign};

use crate::integers::natural_numbers::NaturalNumber;
use crate::{TwosType, TwosTypeSigned};

impl ShrAssign<TwosType> for NaturalNumber {
    fn shr_assign(&mut self, rhs: TwosType) {
        self.twos -= rhs as TwosTypeSigned;
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl ShlAssign<TwosType> for NaturalNumber {
    fn shl_assign(&mut self, rhs: TwosType) {
        self.twos += rhs as TwosTypeSigned;
    }
}
