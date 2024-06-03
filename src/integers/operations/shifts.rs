use crate::integers::Integer;
use crate::integers::Integer::{NonZero, Zero};
use crate::TwosType;
use std::ops::{ShlAssign, ShrAssign};

impl ShrAssign<TwosType> for Integer {
    fn shr_assign(&mut self, rhs: TwosType) {
        match self {
            Zero => {}
            NonZero(ref mut x, sign) => {
                // Round down if the number is negative and the right shift would round up.
                let round_down = *sign && rhs > x.trailing_zeros();

                x.shr_assign(rhs);
                if round_down {
                    x.increment();
                }

                self.reduce();
            }
        }
    }
}

impl ShlAssign<TwosType> for Integer {
    fn shl_assign(&mut self, rhs: TwosType) {
        match self {
            Zero => {}
            NonZero(x, _) => x.shl_assign(rhs),
        }
    }
}
