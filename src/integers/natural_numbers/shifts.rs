use crate::integers::natural_numbers::NaturalNumber;
use crate::TwosType;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::{ShlAssign, ShrAssign};

impl ShrAssign<TwosType> for NaturalNumber {
    fn shr_assign(&mut self, rhs: TwosType) {
        match self.twos.cmp(&rhs) {
            Greater => {
                self.twos -= rhs;
            }
            Equal => {
                self.twos = 0;
            }
            Less => {
                self.odd_part.shr_assign(rhs - self.twos);
                self.twos = 0;
            }
        }
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl ShlAssign<TwosType> for NaturalNumber {
    fn shl_assign(&mut self, rhs: TwosType) {
        self.twos += rhs;
    }
}
