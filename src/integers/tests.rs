mod tests {
    use num_bigint::BigInt;

    use crate::integers::Integer;

    #[test]
    fn to_from_bigint() {
        let value = BigInt::from(148u16);
        let binary = Integer::from(&value);
        assert_eq!(value, (&binary).into());
    }

    #[test]
    fn test_addition() {
        let a = Integer::from(&BigInt::from(18u16));
        let b = Integer::from(&BigInt::from(15u16));
        let c = a + &b;
        assert_eq!(c, Integer::from(&BigInt::from(33u16)));

        let d = Integer::from(&BigInt::from(-18i16));
        let e = Integer::from(&BigInt::from(15i16));
        let f = d.clone() + &e;
        assert_eq!(f, Integer::from(&BigInt::from(-3i16)));
        let f = e.clone() + &d;
        assert_eq!(f, Integer::from(&BigInt::from(-3i16)));

        let g = Integer::from(&BigInt::from(-18i16));
        let h = Integer::from(&BigInt::from(-15i16));
        let i = g + &h;
        assert_eq!(i, Integer::from(&BigInt::from(-33i16)));

        let mut a = Integer::from(&BigInt::from(24u16));
        let b = Integer::from(&BigInt::from(15u16));
        a += &b;
        assert_eq!(a, Integer::from(&BigInt::from(24u16 + 15u16)));
        let c = Integer::from(&BigInt::from(-24i16));
        a += &c;
        assert_eq!(a, Integer::from(&BigInt::from(24i16 + 15i16 - 24i16)));
    }

    #[test]
    fn test_subtraction() {
        let a = Integer::from(&BigInt::from(24u16));
        let b = Integer::from(&BigInt::from(15u16));
        let c = a.clone() - &b;
        assert_eq!(c, Integer::from(&BigInt::from(9u16)));

        let c = b.clone() - &a;
        assert_eq!(c, Integer::from(&BigInt::from(-9)));

        let d = Integer::from(&BigInt::from(-24i16));
        let e = Integer::from(&BigInt::from(15i16));
        let f = d.clone() - &e;
        assert_eq!(f, Integer::from(&BigInt::from(-39i16)));
        let f = e.clone() - &d;
        assert_eq!(f, Integer::from(&BigInt::from(39i16)));

        let g = Integer::from(&BigInt::from(-24i16));
        let h = Integer::from(&BigInt::from(-15i16));
        let i = g.clone() - &h;
        assert_eq!(i, Integer::from(&BigInt::from(-9i16)));
    }

    #[test]
    fn test_multiplication() {
        let mut a = Integer::from(&BigInt::from(24u16));
        let b = Integer::from(&BigInt::from(15u16));
        a *= &b;
        assert_eq!(a, Integer::from(&BigInt::from(24u16 * 15u16)));

        let mut a = Integer::from(&BigInt::from(-24i16));
        let b = Integer::from(&BigInt::from(15i16));
        a *= &b;
        assert_eq!(a, Integer::from(&BigInt::from(-24i16 * 15i16)));

        let mut a = Integer::from(&BigInt::from(-24i16));
        let b = Integer::from(&BigInt::from(-15i16));
        a *= &b;
        assert_eq!(a, Integer::from(&BigInt::from(24i16 * 15i16)));
    }

    #[test]
    fn test_shl() {
        let mut a = Integer::from(&BigInt::from(24u16));
        a <<= 1;
        assert_eq!(a, Integer::from(&BigInt::from(24u16 << 1)));
        a <<= 1;
        assert_eq!(a, Integer::from(&BigInt::from(24u16 << 2)));

        let mut a = Integer::from(&BigInt::from(-24i16));
        a <<= 1;
        assert_eq!(a, Integer::from(&BigInt::from(-24i16 << 1)));
        a <<= 1;
        assert_eq!(a, Integer::from(&BigInt::from(-24i16 << 2)));
    }

    #[test]
    fn test_shr() {
        let mut a = Integer::from(&BigInt::from(24u16));
        a >>= 1;
        assert_eq!(a, Integer::from(&BigInt::from(24u16 >> 1)));
        a >>= 1;
        assert_eq!(a, Integer::from(&BigInt::from(24u16 >> 2)));
        a >>= 1;
        assert_eq!(a, Integer::from(&BigInt::from(24u16 >> 3)));
        a >>= 1;
        assert_eq!(a, Integer::from(&BigInt::from(24u16 >> 4)));
    }

    #[test]
    fn test_cmp() {
        let a = Integer::from(&BigInt::from(17u16));
        let b = Integer::from(&BigInt::from(15u16));
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
        assert_eq!(b.cmp(&a), std::cmp::Ordering::Less);
        assert_eq!(a.cmp(&a), std::cmp::Ordering::Equal);

        let a = Integer::from(&BigInt::from(-18i16));
        let b = Integer::from(&BigInt::from(24i16));
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
        assert_eq!(b.cmp(&a), std::cmp::Ordering::Greater);
        assert_eq!(a.cmp(&a), std::cmp::Ordering::Equal);
    }
}
