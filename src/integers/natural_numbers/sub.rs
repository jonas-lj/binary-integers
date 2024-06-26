use crate::integers::natural_numbers::NaturalNumber;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::{Shl, ShlAssign, Sub, SubAssign};

impl SubAssign<&NaturalNumber> for NaturalNumber {
    fn sub_assign(&mut self, rhs: &NaturalNumber) {
        match self.twos.cmp(&rhs.twos) {
            Less => {
                self.odd_part -= (&rhs.odd_part).shl(rhs.twos - self.twos);
            }
            Equal => {
                self.odd_part -= &rhs.odd_part;
                self.reduce();
            }
            Greater => {
                self.odd_part.shl_assign(self.twos - rhs.twos);
                self.odd_part -= &rhs.odd_part;
                self.twos = rhs.twos;
            }
        }
    }
}

impl Sub<&NaturalNumber> for NaturalNumber {
    type Output = NaturalNumber;

    fn sub(self, rhs: &NaturalNumber) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}
