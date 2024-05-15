use crate::integers::Integer;
use crate::integers::Integer::{NonZero, Zero};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Zero, Zero) => Equal,
            (Zero, NonZero(_, false)) => Less,
            (NonZero(_, false), Zero) => Greater,
            (Zero, NonZero(_, true)) => Greater,
            (NonZero(_, true), Zero) => Less,
            (NonZero(_, true), NonZero(_, false)) => Less,
            (NonZero(_, false), NonZero(_, true)) => Greater,
            (NonZero(x, false), NonZero(y, false)) => x.cmp(y),
            (NonZero(x, true), NonZero(y, true)) => x.cmp(y).reverse(),
        }
    }
}
