use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ops::Shl;

use crate::integers::natural_numbers::NaturalNumber;

impl PartialOrd<Self> for NaturalNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NaturalNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.bits().cmp(&other.bits()) {
            Less => Less,
            Greater => Greater,
            Equal => match self.twos.cmp(&other.twos) {
                Less => self.value.cmp(&(&other.value).shl(other.twos - self.twos)),
                Equal => self.value.cmp(&other.value),
                Greater => (&self.value).shl(self.twos - other.twos).cmp(&other.value),
            },
        }
    }
}
