use crate::integers::natural_numbers::NaturalNumber;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::{AddAssign, Shl, ShlAssign};

impl AddAssign<&NaturalNumber> for NaturalNumber {
    fn add_assign(&mut self, rhs: &NaturalNumber) {
        match self.twos.cmp(&rhs.twos) {
            Less => {
                self.value += (&rhs.value).shl(rhs.twos - self.twos);
            }
            Equal => {
                self.value += &rhs.value;
                self.reduce();
            }
            Greater => {
                self.value.shl_assign(self.twos - rhs.twos);
                self.value += &rhs.value;
                self.twos = rhs.twos;
            }
        }
    }
}
