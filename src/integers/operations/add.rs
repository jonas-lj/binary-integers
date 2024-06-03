use crate::integers::Integer;
use crate::integers::Integer::{NonZero, Zero};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::mem;
use std::ops::{Add, AddAssign, SubAssign};

impl AddAssign<&Integer> for Integer {
    fn add_assign(&mut self, rhs: &Integer) {
        match (&mut *self, rhs) {
            (&mut Zero, _) => {
                let _ = mem::replace(self, rhs.clone());
            }
            (_, Zero) => {}
            (&mut NonZero(ref mut x, false), NonZero(y, false))
            | (&mut NonZero(ref mut x, true), NonZero(y, true)) => x.add_assign(y),
            (&mut NonZero(ref mut x, ref mut sign), NonZero(y, true))
            | (&mut NonZero(ref mut x, ref mut sign), NonZero(y, false)) => match (*x).cmp(y) {
                Less => {
                    let _ = mem::replace(x, y.clone() - x);
                    let _ = mem::replace(sign, !*sign);
                }
                Equal => {
                    let _ = mem::take(self);
                }
                Greater => {
                    x.sub_assign(y);
                }
            },
        }
    }
}

impl Add<&Integer> for Integer {
    type Output = Integer;

    fn add(self, rhs: &Integer) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl Add<Integer> for Integer {
    type Output = Integer;

    fn add(self, rhs: Integer) -> Self::Output {
        let mut result = self;
        result += &rhs;
        result
    }
}

impl num_traits::Zero for Integer {
    fn zero() -> Self {
        Integer::Zero
    }

    fn is_zero(&self) -> bool {
        matches!(self, Zero)
    }
}
