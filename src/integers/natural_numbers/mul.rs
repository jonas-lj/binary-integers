use crate::integers::natural_numbers::NaturalNumber;
use std::ops::MulAssign;

impl MulAssign<&NaturalNumber> for NaturalNumber {
    fn mul_assign(&mut self, rhs: &NaturalNumber) {
        self.twos += rhs.twos;
        self.odd_part *= &rhs.odd_part;
    }
}
