#![no_std]

use one_of::{case, one_of};

macro_rules! not {
    ($t:ty) => {
        Option::<$t>::None
    };
}

#[test]
fn u32_or_char() {
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

#[test]
fn case() {
    case!(<one_of!(i8, i64, u8, u64)>::from(42u64),
        // i8
        _ => {
            panic!("not i8");
        };

        // i64
        _ => {
            panic!("not i64");
        };

        // u8
        _ => {
            panic!("not u8");
        };

        // u64
        0 ..= 41 => {
            panic!("not less than 42");
        }
        n => {
            assert_eq!(n, 42);
        };
    );
}

#[test]
fn case_guards() {
    case!(<one_of!(bool, &str, i64)>::from("Hello, world!"),
        // bool
        _ => {
            panic!("not bool");
        };

        // &str
        s if s.starts_with("Hello, ") => {
            assert_eq!(&s[7 ..], "world!");
        }
        _ => {
            panic!("not other strings");
        };

        // i64
        _ => {
            panic!("not i64");
        };
    );
}
