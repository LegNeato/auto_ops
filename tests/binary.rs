use auto_ops::{impl_op, impl_op_commutative, impl_op_ex, impl_op_ex_commutative};

mod kong {
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Barrel<T> {
        pub bananas: T,
    }

    impl<T> Barrel<T> {
        pub fn new(bananas: T) -> Barrel<T> {
            Barrel { bananas }
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Donkey {
        pub bananas: i32,
    }

    impl Donkey {
        pub fn new(bananas: i32) -> Donkey {
            Donkey { bananas }
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Diddy {
        pub bananas: i32,
    }

    impl Diddy {
        pub fn new(bananas: i32) -> Diddy {
            Diddy { bananas }
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct Dixie {
        pub bananas: i32,
    }

    impl Dixie {
        pub fn new(bananas: i32) -> Dixie {
            Dixie { bananas }
        }
    }
}

mod impl_op_operators {
    use super::*;

    impl_op!(+ |a: kong::Donkey, b: kong::Diddy| -> kong::Dixie { kong::Dixie::new(a.bananas + b.bananas) });
    #[test]
    fn add() {
        assert_eq!(
            kong::Dixie::new(1 + 2),
            kong::Donkey::new(1) + kong::Diddy::new(2)
        );
    }

    impl_op!(-|a: kong::Donkey, b: kong::Diddy| -> kong::Dixie {
        kong::Dixie::new(a.bananas - b.bananas)
    });
    #[test]
    fn sub() {
        assert_eq!(
            kong::Dixie::new(1 - 2),
            kong::Donkey::new(1) - kong::Diddy::new(2)
        );
    }

    impl_op!(*|a: kong::Donkey, b: kong::Diddy| -> kong::Dixie {
        kong::Dixie::new(a.bananas * b.bananas)
    });
    #[test]
    fn mul() {
        assert_eq!(
            kong::Dixie::new(1 * 2),
            kong::Donkey::new(1) * kong::Diddy::new(2)
        );
    }

    impl_op!(/ |a: kong::Donkey, b: kong::Diddy| -> kong::Dixie { kong::Dixie::new(a.bananas / b.bananas) });
    #[test]
    fn div() {
        assert_eq!(
            kong::Dixie::new(1 / 2),
            kong::Donkey::new(1) / kong::Diddy::new(2)
        );
    }
    impl_op!(% |a: kong::Donkey, b: kong::Diddy| -> kong::Dixie { kong::Dixie::new(a.bananas % b.bananas) });
    #[test]
    fn rem() {
        assert_eq!(
            kong::Dixie::new(1 % 2),
            kong::Donkey::new(1) % kong::Diddy::new(2)
        );
    }

    impl_op!(&|a: kong::Donkey, b: kong::Diddy| -> kong::Dixie {
        kong::Dixie::new(a.bananas & b.bananas)
    });
    #[test]
    fn bitand() {
        assert_eq!(
            kong::Dixie::new(1 & 2),
            kong::Donkey::new(1) & kong::Diddy::new(2)
        );
    }
    impl_op!(| |a: kong::Donkey, b: kong::Diddy| -> kong::Dixie { kong::Dixie::new(a.bananas | b.bananas) });
    #[test]
    fn bitor() {
        assert_eq!(
            kong::Dixie::new(1 | 2),
            kong::Donkey::new(1) | kong::Diddy::new(2)
        );
    }
    impl_op!(^ |a: kong::Donkey, b: kong::Diddy| -> kong::Dixie { kong::Dixie::new(a.bananas ^ b.bananas) });
    #[test]
    fn bitxor() {
        assert_eq!(
            kong::Dixie::new(1 ^ 2),
            kong::Donkey::new(1) ^ kong::Diddy::new(2)
        );
    }

    impl_op!(<< |a: kong::Donkey, b: kong::Diddy| -> kong::Dixie { kong::Dixie::new(a.bananas << b.bananas) });
    #[test]
    fn shl() {
        assert_eq!(
            kong::Dixie::new(1 << 2),
            kong::Donkey::new(1) << kong::Diddy::new(2)
        );
    }
    impl_op!(>> |a: kong::Donkey, b: kong::Diddy| -> kong::Dixie { kong::Dixie::new(a.bananas >> b.bananas) });
    #[test]
    fn shr() {
        assert_eq!(
            kong::Dixie::new(1 >> 2),
            kong::Donkey::new(1) >> kong::Diddy::new(2)
        );
    }
}

mod impl_op_variants {
    use super::*;

    impl_op!(-|a: kong::Diddy, b: kong::Dixie| -> kong::Donkey {
        kong::Donkey::new(a.bananas - b.bananas)
    });
    #[test]
    fn owned_owned() {
        assert_eq!(
            kong::Donkey::new(1 - 2),
            kong::Diddy::new(1) - kong::Dixie::new(2)
        );
    }

    impl_op!(-|a: kong::Diddy, b: &kong::Dixie| -> kong::Donkey {
        kong::Donkey::new(a.bananas - b.bananas)
    });
    #[test]
    fn owned_borrowed() {
        assert_eq!(
            kong::Donkey::new(1 - 2),
            kong::Diddy::new(1) - &kong::Dixie::new(2)
        );
    }

    impl_op!(-|a: &kong::Diddy, b: kong::Dixie| -> kong::Donkey {
        kong::Donkey::new(a.bananas - b.bananas)
    });
    #[test]
    fn borrowed_owned() {
        assert_eq!(
            kong::Donkey::new(1 - 2),
            &kong::Diddy::new(1) - kong::Dixie::new(2)
        );
    }

    impl_op!(-|a: &kong::Diddy, b: &kong::Dixie| -> kong::Donkey {
        kong::Donkey::new(a.bananas - b.bananas)
    });
    #[test]
    fn borrowed_borrowed() {
        assert_eq!(
            kong::Donkey::new(1 - 2),
            &kong::Diddy::new(1) - &kong::Dixie::new(2)
        );
    }
}

mod impl_op_commutative_variants {
    use super::*;

    impl_op_commutative!(+ |a: kong::Diddy, b: kong::Dixie| -> kong::Donkey { kong::Donkey::new(a.bananas + b.bananas) });
    #[test]
    fn owned_owned() {
        assert_eq!(
            kong::Donkey::new(1 + 2),
            kong::Diddy::new(1) + kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Donkey::new(2 + 1),
            kong::Dixie::new(1) + kong::Diddy::new(2)
        );
    }

    impl_op_commutative!(+ |a: kong::Diddy, b: &kong::Dixie| -> kong::Donkey { kong::Donkey::new(a.bananas + b.bananas) });
    #[test]
    fn owned_borrowed() {
        assert_eq!(
            kong::Donkey::new(1 + 2),
            kong::Diddy::new(1) + &kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Donkey::new(2 + 1),
            &kong::Dixie::new(1) + kong::Diddy::new(2)
        );
    }

    impl_op_commutative!(*|a: &kong::Diddy, b: kong::Dixie| -> kong::Donkey {
        kong::Donkey::new(a.bananas * b.bananas)
    });
    #[test]
    fn borrowed_owned() {
        assert_eq!(
            kong::Donkey::new(1 * 2),
            &kong::Diddy::new(1) * kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Donkey::new(2 * 1),
            kong::Dixie::new(1) * &kong::Diddy::new(2)
        );
    }

    impl_op_commutative!(*|a: &kong::Diddy, b: &kong::Dixie| -> kong::Donkey {
        kong::Donkey::new(a.bananas * b.bananas)
    });
    #[test]
    fn borrowed_borrowed() {
        assert_eq!(
            kong::Donkey::new(1 * 2),
            &kong::Diddy::new(1) * &kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Donkey::new(2 * 1),
            &kong::Dixie::new(1) * &kong::Diddy::new(2)
        );
    }
}

mod impl_op_ex_variants {
    use super::*;

    impl_op_ex!(/ |a: kong::Diddy, b: kong::Dixie| -> kong::Donkey { kong::Donkey::new(a.bananas / b.bananas) });
    #[test]
    fn owned_owned() {
        assert_eq!(
            kong::Donkey::new(1 / 2),
            kong::Diddy::new(1) / kong::Dixie::new(2)
        );
    }

    impl_op_ex!(% |a: kong::Diddy, b: &kong::Dixie| -> kong::Donkey { kong::Donkey::new(a.bananas % b.bananas) });
    #[test]
    fn owned_borrowed() {
        assert_eq!(
            kong::Donkey::new(1 % 2),
            kong::Diddy::new(1) % &kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Donkey::new(1 % 2),
            kong::Diddy::new(1) % kong::Dixie::new(2)
        );
    }

    impl_op_ex!(&|a: &kong::Diddy, b: kong::Dixie| -> kong::Donkey {
        kong::Donkey::new(a.bananas & b.bananas)
    });
    #[test]
    fn borrowed_owned() {
        assert_eq!(
            kong::Donkey::new(1 & 2),
            &kong::Diddy::new(1) & kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Donkey::new(1 & 2),
            kong::Diddy::new(1) & kong::Dixie::new(2)
        );
    }

    impl_op_ex!(| |a: &kong::Diddy, b: &kong::Dixie| -> kong::Donkey { kong::Donkey::new(a.bananas | b.bananas) });
    #[test]
    fn borrowed_borrowed() {
        assert_eq!(
            kong::Donkey::new(1 | 2),
            &kong::Diddy::new(1) | &kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Donkey::new(1 | 2),
            &kong::Diddy::new(1) | kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Donkey::new(1 | 2),
            kong::Diddy::new(1) | &kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Donkey::new(1 | 2),
            kong::Diddy::new(1) | kong::Dixie::new(2)
        );
    }
}

mod impl_op_ex_commutative_variants {
    use super::*;

    impl_op_ex_commutative!(+ |a: kong::Donkey, b: kong::Dixie| -> kong::Diddy { kong::Diddy::new(a.bananas + b.bananas) });
    #[test]
    fn owned_owned_ex_commutative() {
        assert_eq!(
            kong::Diddy::new(1 + 2),
            kong::Donkey::new(1) + kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Diddy::new(2 + 1),
            kong::Dixie::new(1) + kong::Donkey::new(2)
        );
    }

    impl_op_ex_commutative!(*|a: kong::Donkey, b: &kong::Dixie| -> kong::Diddy {
        kong::Diddy::new(a.bananas * b.bananas)
    });
    #[test]
    fn owned_borrowed() {
        assert_eq!(
            kong::Diddy::new(1 * 2),
            kong::Donkey::new(1) * &kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Diddy::new(1 * 2),
            kong::Donkey::new(1) * kong::Dixie::new(2)
        );

        assert_eq!(
            kong::Diddy::new(2 * 1),
            &kong::Dixie::new(1) * kong::Donkey::new(2)
        );
        assert_eq!(
            kong::Diddy::new(2 * 1),
            kong::Dixie::new(1) * kong::Donkey::new(2)
        );
    }

    impl_op_ex_commutative!(&|a: &kong::Donkey, b: kong::Dixie| -> kong::Diddy {
        kong::Diddy::new(a.bananas & b.bananas)
    });
    #[test]
    fn borrowed_owned() {
        assert_eq!(
            kong::Diddy::new(1 & 2),
            &kong::Donkey::new(1) & kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Diddy::new(1 & 2),
            kong::Donkey::new(1) & kong::Dixie::new(2)
        );

        assert_eq!(
            kong::Diddy::new(2 & 1),
            kong::Dixie::new(1) & &kong::Donkey::new(2)
        );
        assert_eq!(
            kong::Diddy::new(2 & 1),
            kong::Dixie::new(1) & kong::Donkey::new(2)
        );
    }

    impl_op_ex_commutative!(| |a: &kong::Donkey, b: &kong::Dixie| -> kong::Diddy { kong::Diddy::new(a.bananas | b.bananas) });
    #[test]
    fn borrowed_borrowed() {
        assert_eq!(
            kong::Diddy::new(1 | 2),
            &kong::Donkey::new(1) | &kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Diddy::new(1 | 2),
            &kong::Donkey::new(1) | kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Diddy::new(1 | 2),
            kong::Donkey::new(1) | &kong::Dixie::new(2)
        );
        assert_eq!(
            kong::Diddy::new(1 | 2),
            kong::Donkey::new(1) | kong::Dixie::new(2)
        );

        assert_eq!(
            kong::Diddy::new(2 | 1),
            &kong::Dixie::new(1) | &kong::Donkey::new(2)
        );
        assert_eq!(
            kong::Diddy::new(2 | 1),
            kong::Dixie::new(1) | &kong::Donkey::new(2)
        );
        assert_eq!(
            kong::Diddy::new(2 | 1),
            &kong::Dixie::new(1) | kong::Donkey::new(2)
        );
        assert_eq!(
            kong::Diddy::new(2 | 1),
            kong::Dixie::new(1) | kong::Donkey::new(2)
        );
    }
}

mod generics {
    use super::*;

    impl_op!(+ |a: kong::Barrel<i32>, b: kong::Barrel<u32>| -> kong::Barrel<f32> { kong::Barrel::new((a.bananas + b.bananas as i32) as f32) });
    #[test]
    fn impl_op() {
        assert_eq!(
            kong::Barrel::new((1 + 2) as f32),
            kong::Barrel::new(1) + kong::Barrel::new(2u32)
        );
    }

    impl_op_commutative!(
        *|a: kong::Barrel<i32>, b: kong::Barrel<u32>| -> kong::Barrel<f32> {
            kong::Barrel::new((a.bananas * b.bananas as i32) as f32)
        }
    );
    #[test]
    fn impl_op_commutative() {
        assert_eq!(
            kong::Barrel::new((1 * 2) as f32),
            kong::Barrel::new(1) * kong::Barrel::new(2u32)
        );
        assert_eq!(
            kong::Barrel::new((2 * 1) as f32),
            kong::Barrel::new(1u32) * kong::Barrel::new(2)
        );
    }

    impl_op_ex!(
        -|a: &kong::Barrel<i32>, b: &kong::Barrel<u32>| -> kong::Barrel<f32> {
            kong::Barrel::new((a.bananas - b.bananas as i32) as f32)
        }
    );
    #[test]
    fn impl_op_ex() {
        assert_eq!(
            kong::Barrel::new((1 - 2) as f32),
            kong::Barrel::new(1) - kong::Barrel::new(2u32)
        );
        assert_eq!(
            kong::Barrel::new((1 - 2) as f32),
            kong::Barrel::new(1) - &kong::Barrel::new(2u32)
        );
        assert_eq!(
            kong::Barrel::new((1 - 2) as f32),
            &kong::Barrel::new(1) - kong::Barrel::new(2u32)
        );
        assert_eq!(
            kong::Barrel::new((1 - 2) as f32),
            &kong::Barrel::new(1) - &kong::Barrel::new(2u32)
        );
    }

    impl_op_ex_commutative!(
        &|a: &kong::Barrel<i32>, b: &kong::Barrel<u32>| -> kong::Barrel<f32> {
            kong::Barrel::new((a.bananas & b.bananas as i32) as f32)
        }
    );
    #[test]
    fn impl_op_ex_commutative() {
        assert_eq!(
            kong::Barrel::new((1 & 2) as f32),
            kong::Barrel::new(1) & kong::Barrel::new(2u32)
        );
        assert_eq!(
            kong::Barrel::new((1 & 2) as f32),
            kong::Barrel::new(1) & &kong::Barrel::new(2u32)
        );
        assert_eq!(
            kong::Barrel::new((1 & 2) as f32),
            &kong::Barrel::new(1) & kong::Barrel::new(2u32)
        );
        assert_eq!(
            kong::Barrel::new((1 & 2) as f32),
            &kong::Barrel::new(1) & &kong::Barrel::new(2u32)
        );

        assert_eq!(
            kong::Barrel::new((2 & 1) as f32),
            kong::Barrel::new(1u32) & kong::Barrel::new(2)
        );
        assert_eq!(
            kong::Barrel::new((2 & 1) as f32),
            kong::Barrel::new(1u32) & &kong::Barrel::new(2)
        );
        assert_eq!(
            kong::Barrel::new((2 & 1) as f32),
            &kong::Barrel::new(1u32) & kong::Barrel::new(2)
        );
        assert_eq!(
            kong::Barrel::new((2 & 1) as f32),
            &kong::Barrel::new(1u32) & &kong::Barrel::new(2)
        );
    }
}

mod generic_params {
    use super::*;

    // Using i8/u8 to avoid conflicting with above impls
    trait Marker {
        fn from_i32(v: i32) -> Self;
    }
    impl Marker for u8 {
        fn from_i32(v: i32) -> Self {
            v as u8
        }
    }
    impl Marker for i8 {
        fn from_i32(v: i32) -> Self {
            v as i8
        }
    }

    impl_op!(
        +
        <T: ::std::ops::Add<T> + Marker>
        |a: kong::Barrel<T>, b: kong::Barrel<T>| -> kong::Barrel<<T as ::std::ops::Add<T>>::Output> {
            kong::Barrel::new(a.bananas + b.bananas)
        }
    );
    #[test]
    fn impl_op() {
        assert_eq!(
            kong::Barrel::new(3u8),
            kong::Barrel::new(1u8) + kong::Barrel::new(2u8)
        );
        assert_eq!(
            kong::Barrel::new(-3i8),
            kong::Barrel::new(-1i8) + kong::Barrel::new(-2i8)
        );
    }

    impl_op_commutative!(
        *
        <T: ::std::ops::Mul<T> + Marker>
        |a: kong::Barrel<i32>, b: kong::Barrel<T>| -> kong::Barrel<<T as ::std::ops::Mul<T>>::Output> {
            kong::Barrel::new(T::from_i32(a.bananas) * b.bananas)
        }
    );
    #[test]
    fn impl_op_commutative() {
        assert_eq!(
            kong::Barrel::new(6u8),
            kong::Barrel::new(3i32) * kong::Barrel::new(2u8)
        );
        assert_eq!(
            kong::Barrel::new(6u8),
            kong::Barrel::new(3u8) * kong::Barrel::new(2i32)
        );
        assert_eq!(
            kong::Barrel::new(-6i8),
            kong::Barrel::new(-3i32) * kong::Barrel::new(2i8)
        );
        assert_eq!(
            kong::Barrel::new(-6i8),
            kong::Barrel::new(-3i8) * kong::Barrel::new(2i32)
        );
    }

    impl_op_ex!(
        -
        <T: ::std::ops::Sub<T> + Marker + Copy>
        |a: &kong::Barrel<T>, b: &kong::Barrel<T>| -> kong::Barrel<<T as ::std::ops::Sub<T>>::Output> {
            kong::Barrel::new(a.bananas - b.bananas)
        }
    );
    #[test]
    fn impl_op_ex() {
        assert_eq!(
            kong::Barrel::new(3u8),
            kong::Barrel::new(5u8) - kong::Barrel::new(2u8)
        );
        assert_eq!(
            kong::Barrel::new(3u8),
            kong::Barrel::new(5u8) - &kong::Barrel::new(2u8)
        );
        assert_eq!(
            kong::Barrel::new(3u8),
            &kong::Barrel::new(5u8) - kong::Barrel::new(2u8)
        );
        assert_eq!(
            kong::Barrel::new(3u8),
            &kong::Barrel::new(5u8) - &kong::Barrel::new(2u8)
        );
        assert_eq!(
            kong::Barrel::new(-7i8),
            kong::Barrel::new(-5i8) - kong::Barrel::new(2i8)
        );
        assert_eq!(
            kong::Barrel::new(-7i8),
            kong::Barrel::new(-5i8) - &kong::Barrel::new(2i8)
        );
        assert_eq!(
            kong::Barrel::new(-7i8),
            &kong::Barrel::new(-5i8) - kong::Barrel::new(2i8)
        );
        assert_eq!(
            kong::Barrel::new(-7i8),
            &kong::Barrel::new(-5i8) - &kong::Barrel::new(2i8)
        );
    }

    impl_op_ex_commutative!(
        &
        <T: ::std::ops::BitAnd<T> + Marker + Copy>
        |a: &kong::Barrel<i32>, b: &kong::Barrel<T>| -> kong::Barrel<<T as ::std::ops::BitAnd<T>>::Output> {
            kong::Barrel::new(T::from_i32(a.bananas) & b.bananas)
        }
    );
    #[test]
    fn impl_op_ex_commutative() {
        // u8 as first operand
        assert_eq!(
            kong::Barrel::new((1 & 2) as u8),
            kong::Barrel::new(1u8) & kong::Barrel::new(2i32)
        );
        assert_eq!(
            kong::Barrel::new((1 & 2) as u8),
            kong::Barrel::new(1u8) & &kong::Barrel::new(2i32)
        );
        assert_eq!(
            kong::Barrel::new((1 & 2) as u8),
            &kong::Barrel::new(1u8) & kong::Barrel::new(2i32)
        );
        assert_eq!(
            kong::Barrel::new((1 & 2) as u8),
            &kong::Barrel::new(1u8) & &kong::Barrel::new(2i32)
        );

        // u8 as second operand
        assert_eq!(
            kong::Barrel::new((2 & 1) as u8),
            kong::Barrel::new(1i32) & kong::Barrel::new(2u8)
        );
        assert_eq!(
            kong::Barrel::new((2 & 1) as u8),
            kong::Barrel::new(1i32) & &kong::Barrel::new(2u8)
        );
        assert_eq!(
            kong::Barrel::new((2 & 1) as u8),
            &kong::Barrel::new(1i32) & kong::Barrel::new(2u8)
        );
        assert_eq!(
            kong::Barrel::new((2 & 1) as u8),
            &kong::Barrel::new(1i32) & &kong::Barrel::new(2u8)
        );

        // i8 as first operand
        assert_eq!(
            kong::Barrel::new((1 & 2) as i8),
            kong::Barrel::new(1i8) & kong::Barrel::new(2i32)
        );
        assert_eq!(
            kong::Barrel::new((1 & 2) as i8),
            kong::Barrel::new(1i8) & &kong::Barrel::new(2i32)
        );
        assert_eq!(
            kong::Barrel::new((1 & 2) as i8),
            &kong::Barrel::new(1i8) & kong::Barrel::new(2i32)
        );
        assert_eq!(
            kong::Barrel::new((1 & 2) as i8),
            &kong::Barrel::new(1i8) & &kong::Barrel::new(2i32)
        );

        // i8 as second operand
        assert_eq!(
            kong::Barrel::new((2 & 1) as i8),
            kong::Barrel::new(1i32) & kong::Barrel::new(2i8)
        );
        assert_eq!(
            kong::Barrel::new((2 & 1) as i8),
            kong::Barrel::new(1i32) & &kong::Barrel::new(2i8)
        );
        assert_eq!(
            kong::Barrel::new((2 & 1) as i8),
            &kong::Barrel::new(1i32) & kong::Barrel::new(2i8)
        );
        assert_eq!(
            kong::Barrel::new((2 & 1) as i8),
            &kong::Barrel::new(1i32) & &kong::Barrel::new(2i8)
        );
    }
}

mod multiline {
    use super::*;

    impl_op!(-|a: kong::Donkey, b: kong::Barrel<i32>| -> kong::Donkey {
        let ret = kong::Donkey::new(a.bananas - b.bananas);
        ret
    });
    #[test]
    fn impl_op() {
        assert_eq!(
            kong::Donkey::new(1 - 2),
            kong::Donkey::new(1) - kong::Barrel::new(2)
        );
    }

    impl_op_commutative!(+ |a: kong::Donkey, b: kong::Barrel<i32>| -> kong::Donkey {
        let ret = kong::Donkey::new(a.bananas + b.bananas);
        ret
    });
    #[test]
    fn impl_op_commutative() {
        assert_eq!(
            kong::Donkey::new(1 + 2),
            kong::Donkey::new(1) + kong::Barrel::new(2)
        );
        assert_eq!(
            kong::Donkey::new(2 + 1),
            kong::Barrel::new(1) + kong::Donkey::new(2)
        );
    }

    impl_op_ex!(/ |a: &kong::Donkey, b: &kong::Barrel<i32>| -> kong::Donkey {
        let ret = kong::Donkey::new(a.bananas / b.bananas);
        ret
    });
    #[test]
    fn impl_op_ex() {
        assert_eq!(
            kong::Donkey::new(1 / 2),
            kong::Donkey::new(1) / kong::Barrel::new(2)
        );
        assert_eq!(
            kong::Donkey::new(1 / 2),
            kong::Donkey::new(1) / &kong::Barrel::new(2)
        );
        assert_eq!(
            kong::Donkey::new(1 / 2),
            &kong::Donkey::new(1) / kong::Barrel::new(2)
        );
        assert_eq!(
            kong::Donkey::new(1 / 2),
            &kong::Donkey::new(1) / &kong::Barrel::new(2)
        );
    }

    impl_op_ex_commutative!(*|a: &kong::Donkey, b: &kong::Barrel<i32>| -> kong::Donkey {
        let ret = kong::Donkey::new(a.bananas * b.bananas);
        ret
    });
    #[test]
    fn impl_op_ex_commutative() {
        assert_eq!(
            kong::Donkey::new(1 * 2),
            kong::Donkey::new(1) * kong::Barrel::new(2)
        );
        assert_eq!(
            kong::Donkey::new(1 * 2),
            kong::Donkey::new(1) * &kong::Barrel::new(2)
        );
        assert_eq!(
            kong::Donkey::new(1 * 2),
            &kong::Donkey::new(1) * kong::Barrel::new(2)
        );
        assert_eq!(
            kong::Donkey::new(1 * 2),
            &kong::Donkey::new(1) * &kong::Barrel::new(2)
        );

        assert_eq!(
            kong::Donkey::new(2 * 1),
            kong::Barrel::new(1) * kong::Donkey::new(2)
        );
        assert_eq!(
            kong::Donkey::new(2 * 1),
            kong::Barrel::new(1) * &kong::Donkey::new(2)
        );
        assert_eq!(
            kong::Donkey::new(2 * 1),
            &kong::Barrel::new(1) * kong::Donkey::new(2)
        );
        assert_eq!(
            kong::Donkey::new(2 * 1),
            &kong::Barrel::new(1) * &kong::Donkey::new(2)
        );
    }
}

fn do_bitor(a: &kong::Donkey, b: &kong::Dixie) -> kong::Diddy {
    a | b
}

fn do_bitor_2(a: &kong::Dixie, b: &kong::Donkey) -> kong::Diddy {
    a | b
}

#[test]
fn infer_lifetimes() {
    assert_eq!(
        kong::Diddy::new(1 | 2),
        do_bitor(&kong::Donkey::new(1), &kong::Dixie::new(2))
    );
    assert_eq!(
        kong::Diddy::new(2 | 1),
        do_bitor_2(&kong::Dixie::new(1), &kong::Donkey::new(2))
    );
}

mod impl_op_sequence_types {
    use super::*;

    impl_op!(+ |a: kong::Donkey, b: (i32, i32)| -> kong::Donkey { kong::Donkey::new(a.bananas + b.0) });
    #[test]
    fn tuple() {
        assert_eq!(kong::Donkey::new(1 + 2), kong::Donkey::new(1) + (2, 3));
    }

    impl_op!(+ |a: kong::Donkey, b: [i32; 2]| -> kong::Donkey { kong::Donkey::new(a.bananas + b[0]) });
    #[test]
    fn array() {
        assert_eq!(kong::Donkey::new(1 + 2), kong::Donkey::new(1) + [2, 3]);
    }

    impl_op!(+ |a: kong::Donkey, b: &[i32]| -> kong::Donkey { kong::Donkey::new(a.bananas + b[0]) });
    #[test]
    fn slice() {
        assert_eq!(
            kong::Donkey::new(1 + 2),
            kong::Donkey::new(1) + vec![2, 3].as_slice()
        );
    }
}
