use crate::integers::natural_numbers::NaturalNumber;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::{AddAssign, Shl, ShlAssign};

impl AddAssign<&NaturalNumber> for NaturalNumber {
    fn add_assign(&mut self, rhs: &NaturalNumber) {
        match self.twos.cmp(&rhs.twos) {
            Less => {
                self.odd_part += (&rhs.odd_part).shl(rhs.twos - self.twos);
            }
            Equal => {
                self.odd_part += &rhs.odd_part;
                self.reduce();
            }
            Greater => {
                self.odd_part.shl_assign(self.twos - rhs.twos);
                self.odd_part += &rhs.odd_part;
                self.twos = rhs.twos;
            }
        }
    }
}
