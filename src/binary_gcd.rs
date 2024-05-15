use std::cmp::{min, Ordering};
use std::ops::{Shl, ShrAssign};

use num_bigint::BigInt;

use crate::{IntegerTraits, TwosType};

#[derive(Debug)]
pub struct EuclideanAlgorithmOutput<N> {
    pub gcd: N,
    pub x: N,
    pub y: N,
}

impl<N> EuclideanAlgorithmOutput<N> {
    fn flip(self) -> Self {
        Self {
            gcd: self.gcd,
            x: self.y,
            y: self.x,
        }
    }
}

/// Compute the greatest common divisor gcd of a and b. The output also returns the Bezout coefficients
/// x and y such that ax + by = gcd and also the quotients a / gcd and b / gcd.
///
/// This implementation is adapted from this implementation: https://gist.github.com/cmpute/0a09749f76303b24b7961362bee8d988.
pub fn extended_euclidean_algorithm<N: IntegerTraits>(a: &N, b: &N) -> EuclideanAlgorithmOutput<N> {
    if a < b {
        return extended_euclidean_algorithm(b, a).flip();
    }

    assert!(!a.is_negative() && !b.is_negative());

    let mut s = (N::one(), N::zero());
    let mut t = (N::zero(), N::one());

    let mut u = a.clone();
    let mut v = b.clone();

    let u_zeros = u.oddify();
    let v_zeros = v.oddify();
    let zeros = min(u_zeros, v_zeros);

    let mut shifts = u_zeros.abs_diff(v_zeros);
    match u_zeros.cmp(&v_zeros) {
        Ordering::Less => s.0 <<= shifts,
        Ordering::Equal => {}
        Ordering::Greater => t.1 <<= shifts,
    }

    while u != v {
        let zeros;
        if u > v {
            u -= &v;
            zeros = u.oddify();
            t.0 += &t.1;
            t.1 <<= zeros;
            s.0 += &s.1;
            s.1 <<= zeros;
        } else {
            v -= &u;
            zeros = v.oddify();
            t.1 += &t.0;
            t.0 <<= zeros;
            s.1 += &s.0;
            s.0 <<= zeros;
        }
        shifts += zeros;
    }

    t.1 += &t.0;
    s.1 += &s.0;

    for _ in 0..shifts {
        if s.0.is_odd() | t.0.is_odd() {
            s.0 += &s.1;
            t.0 += &t.1;
        }
        s.0 >>= 1;
        t.0 >>= 1;
    }

    if s.0.cmp_shifted(1, &s.1).is_gt() {
        s.0 -= &s.1;
        t.0 -= &t.1;
    }

    u <<= zeros;

    EuclideanAlgorithmOutput {
        gcd: u,
        x: s.0,
        y: -t.0,
    }
}

impl IntegerTraits for BigInt {
    fn trailing_zeros(&self) -> TwosType {
        BigInt::trailing_zeros(self).unwrap() as TwosType
    }

    fn oddify(&mut self) -> TwosType {
        let zeros = BigInt::trailing_zeros(self).unwrap();
        self.shr_assign(zeros);
        zeros as TwosType
    }

    fn is_negative(&self) -> bool {
        num_traits::Signed::is_negative(self)
    }

    fn is_odd(&self) -> bool {
        num_integer::Integer::is_odd(self)
    }

    fn cmp_shifted(&self, exp: u16, other: &Self) -> Ordering {
        self.shl(exp).cmp(other)
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use num_integer::Integer as OtherInteger;
    use rand::{thread_rng, RngCore};
    use std::io;
    use std::io::Write;

    use crate::binary_gcd::extended_euclidean_algorithm;
    use crate::integers::Integer;

    #[test]
    fn test_xgcd_binary() {
        let mut rnd = thread_rng();
        let mut a_bytes = vec![0u8; 1024];
        let mut b_bytes = vec![0u8; 1024];
        let tests = 20;

        for _ in 0..tests {
            rnd.fill_bytes(&mut a_bytes);
            let a = BigInt::from_bytes_be(num_bigint::Sign::Plus, &a_bytes);

            rnd.fill_bytes(&mut b_bytes);
            let b = BigInt::from_bytes_be(num_bigint::Sign::Plus, &b_bytes);

            let a_prime = Integer::from(&a);
            let b_prime = Integer::from(&b);

            let result = extended_euclidean_algorithm::<Integer>(&a_prime, &b_prime);

            let x: BigInt = (&result.x).into();
            let y: BigInt = (&result.y).into();
            let g: BigInt = (&result.gcd).into();

            assert_eq!(g, BigInt::gcd(&a, &b));
            assert_eq!(g, x * a + y * b);
        }
    }

    #[test]
    fn test_xgcd_bigint() {
        let mut rnd = thread_rng();
        let mut a_bytes = vec![0u8; 1024];
        let mut b_bytes = vec![0u8; 1024];
        let tests = 100;

        for _ in 0..tests {
            rnd.fill_bytes(&mut a_bytes);
            let a = BigInt::from_bytes_be(num_bigint::Sign::Plus, &a_bytes);

            rnd.fill_bytes(&mut b_bytes);
            let b = BigInt::from_bytes_be(num_bigint::Sign::Plus, &b_bytes);

            let result = extended_euclidean_algorithm::<BigInt>(&a, &b);

            assert_eq!(result.gcd, BigInt::gcd(&a, &b));
            assert_eq!(result.gcd, result.x * a + result.y * b);
        }
    }
}
