use num_traits::{One, Zero};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, MulAssign, Neg, ShlAssign, ShrAssign, SubAssign};

pub mod binary_gcd;
pub mod integers;

/// Minimal trait for integers used in Stein's algorithm.
pub trait IntegerTraits:
    Clone
    + ShlAssign<TwosType>
    + ShrAssign<TwosType>
    + Neg<Output = Self>
    + PartialOrd
    + for<'a> SubAssign<&'a Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> MulAssign<&'a Self>
    + One
    + Zero
    + Add<Self>
{
    /// Returns the number of trailing zeros in the binary representation.
    fn trailing_zeros(&self) -> TwosType;

    /// Divides by 2 until the number is odd and returns the number of divisions.
    fn oddify(&mut self) -> TwosType;

    fn is_negative(&self) -> bool;

    fn is_odd(&self) -> bool;

    fn is_even(&self) -> bool {
        !self.is_odd()
    }

    /// Compare self * 2^exp with other.
    fn cmp_shifted(&self, exp: u16, other: &Self) -> Ordering;
}

type TwosType = u16;