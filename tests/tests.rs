#![no_std]

use one_of::one_of;

macro_rules! not {
    ($t:ty) => {
        Option::<$t>::None
    };
}

#[test]
fn i32_or_char() {
    let x: one_of!(u32, char) = 42.into();
    assert_eq!(Some(42u32), x.into());
    assert_eq!(not!(char), x.into());
}

#[test]
fn some_type_of_integer() {
    let x: one_of!(i8, i16, i32, i64, u8, u16, u32, u64) = 42.into();
    assert_eq!(not!(i8), x.into());
    assert_eq!(not!(i16), x.into());
    assert_eq!(Some(42i32), x.into());
    assert_eq!(not!(i64), x.into());
    assert_eq!(not!(u8), x.into());
    assert_eq!(not!(u16), x.into());
    assert_eq!(not!(u32), x.into());
    assert_eq!(not!(u64), x.into());
}
