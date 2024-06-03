use crate::integers::Integer;
use crate::integers::Integer::{NonZero, Zero};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::mem;
use std::ops::{AddAssign, Neg, Sub, SubAssign};

impl SubAssign<&Integer> for Integer {
    fn sub_assign(&mut self, rhs: &Integer) {
        match (&mut *self, rhs) {
            (&mut Zero, _) => {
                let _ = mem::replace(self, rhs.neg());
            }
            (_, Zero) => {}
            (&mut NonZero(ref mut x, false), NonZero(y, true))
            | (&mut NonZero(ref mut x, true), NonZero(y, false)) => x.add_assign(y),
            (&mut NonZero(ref mut x, false), NonZero(y, false)) => match (*x).cmp(y) {
                Less => {
                    let new_x = y.clone() - x;
                    let _ = mem::replace(self, NonZero(new_x, true));
                }
                Equal => {
                    let _ = mem::replace(self, Zero);
                }
                Greater => x.sub_assign(y),
            },
            (&mut NonZero(ref mut x, true), NonZero(y, true)) => match (*x).cmp(y) {
                Less => {
                    let new_x = y.clone() - x;
                    let _ = mem::replace(self, NonZero(new_x, false));
                }
                Equal => {
                    let _ = mem::replace(self, Zero);
                }
                Greater => x.sub_assign(y),
            },
        }
    }
}

impl Sub<&Integer> for Integer {
    type Output = Integer;

    fn sub(self, rhs: &Integer) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}
