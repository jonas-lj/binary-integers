mod tests {
    use std::ops::{ShlAssign, ShrAssign};

    use num_bigint::BigInt;
    use rand::Rng;

    use crate::integers::Integer;

    #[test]
    fn to_from_bigint() {
        let value = BigInt::from(148u16);
        let binary = Integer::from(value.clone());
        assert_eq!(value, (&binary).into());
    }

    #[test]
    fn test_cmp() {
        let a = Integer::from(17u16);
        let b = Integer::from(15u16);
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
        assert_eq!(b.cmp(&a), std::cmp::Ordering::Less);
        assert_eq!(a.cmp(&a), std::cmp::Ordering::Equal);

        let a = Integer::from(-18);
        let b = Integer::from(24);
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
        assert_eq!(b.cmp(&a), std::cmp::Ordering::Greater);
        assert_eq!(a.cmp(&a), std::cmp::Ordering::Equal);
    }

    #[test]
    fn random_tests() {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let a_i32 = rng.gen_range(-1000..1000);
            let b_i32 = rng.gen_range(-1000..1000);
            let a = Integer::from(a_i32);
            let b = Integer::from(b_i32);

            let c = a.clone() + &b;
            assert_eq!(c, Integer::from(a_i32 + b_i32));

            let c = a.clone() - &b;
            assert_eq!(c, Integer::from(a_i32 - b_i32));

            let c = a.clone() * b.clone();
            assert_eq!(c, Integer::from(a_i32 * b_i32));

            let mut c = a.clone();
            c.shr_assign(3);
            let expected = a_i32 >> 3;
            assert_eq!(c, Integer::from(expected));

            let mut c = a.clone();
            c.shl_assign(3);
            assert_eq!(c, Integer::from(a_i32 << 3));

            assert_eq!(a.cmp(&b), a_i32.cmp(&b_i32));
        }
    }
}
