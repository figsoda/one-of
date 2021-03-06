//! Macro to represent a type that can be converted either [`From`] or [`TryInto`] the given types
//!
//! This crate only works on the nightly version of Rust
//!
//!
//! ## Usage
//!
//! ```
//! use one_of::{case, one_of};
//!
//! // either `u32` or `char`
//! let x: one_of!(u32, char) = 42.into();
//! assert_eq!(Some(42u32), x.into());
//! assert_eq!(Option::<char>::None, x.into());
//!
//! // some type of integer
//! let x: one_of!(i8, i16, i32, i64, u8, u16, u32, u64) = 42.into();
//! assert_eq!(Option::<i8>::None, x.into());
//! assert_eq!(Option::<i16>::None, x.into());
//! assert_eq!(Some(42i32), x.into());
//! assert_eq!(Option::<i64>::None, x.into());
//! assert_eq!(Option::<u8>::None, x.into());
//! assert_eq!(Option::<u16>::None, x.into());
//! assert_eq!(Option::<u32>::None, x.into());
//! assert_eq!(Option::<u64>::None, x.into());
//!
//! // case macro is the `match` keyword for `one_of` types
//! case!(<one_of!(bool, &str, i64)>::from("Hello, world!"),
//!     // bool
//!     _ => {
//!         panic!("not bool");
//!     };
//!
//!     // &str
//!     s if s.starts_with("Hello, ") => {
//!         assert_eq!(&s[7 ..], "world!");
//!     }
//!     _ => {
//!         panic!("not other strings");
//!     };
//!
//!     // i64
//!     _ => {
//!         panic!("not i64");
//!     };
//! );
//! ```
//!
//!
//! ## Changelog
//!
//! See [CHANGELOG.md](https://github.com/figsoda/one-of/blob/main/CHANGELOG.md)

#![feature(auto_traits, negative_impls)]
#![forbid(unsafe_code)]
#![no_std]

use core::{
    convert::TryInto,
    fmt::{self, Display, Formatter},
};

// https://github.com/rust-lang/rust/issues/30905#issuecomment-173327799
mod internal {
    pub auto trait Different {}
    impl<T> !Different for (T, T) {}
}
use crate::internal::Different;

macro_rules! gen_types {
    ($($n:ident { $($v:ident: $($l:ident,)* @ $(,$r:ident)*;)+ })*) => {
        $(
            #[doc(hidden)]
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            pub enum $n<$($v),+> {
                $($v($v)),+
            }

            impl<$($v: Display),+> Display for $n<$($v),+> {
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                    match self {
                        $(Self::$v(x) => write!(f, "{}", x)),+
                    }
                }
            }

            $(
                impl<$($l,)* $v $(,$r)*> From<$v> for $n<$($l,)* $v $(,$r)*> where
                    $(($v, $l): Different,)* $(($v, $r): Different,)* {
                    fn from(x: $v) -> Self {
                        Self::$v(x)
                    }
                }

                impl<$($l,)* $v $(,$r)*> From<$n<$($l,)* $v $(,$r)*>> for Option<$v> where
                    $(($v, $l): Different,)* $(($v, $r): Different,)* {
                    fn from(x: $n<$($l,)* $v $(,$r)*>) -> Self {
                        match x {
                            $n::$v(x) => Some(x),
                            _ => None,
                        }
                    }
                }

                impl<$($l,)* $v $(,$r)*> TryInto<$v> for $n<$($l,)* $v $(,$r)*> where
                    $(($v, $l): Different,)* $(($v, $r): Different,)* {
                    type Error = ();

                    fn try_into(self) -> Result<$v, Self::Error> {
                        match self {
                            $n::$v(x) => Ok(x),
                            _ => Err(()),
                        }
                    }
                }
            )+
        )*
    }
}

gen_types!(
    OneOf2 {
        A: @, B;
        B: A, @;
    }
    OneOf3 {
        A: @, B, C;
        B: A, @, C;
        C: A, B, @;
    }
    OneOf4 {
        A: @, B, C, D;
        B: A, @, C, D;
        C: A, B, @, D;
        D: A, B, C, @;
    }
    OneOf5 {
        A: @, B, C, D, E;
        B: A, @, C, D, E;
        C: A, B, @, D, E;
        D: A, B, C, @, E;
        E: A, B, C, D, @;
    }
    OneOf6 {
        A: @, B, C, D, E, F;
        B: A, @, C, D, E, F;
        C: A, B, @, D, E, F;
        D: A, B, C, @, E, F;
        E: A, B, C, D, @, F;
        F: A, B, C, D, E, @;
    }
    OneOf7 {
        A: @, B, C, D, E, F, G;
        B: A, @, C, D, E, F, G;
        C: A, B, @, D, E, F, G;
        D: A, B, C, @, E, F, G;
        E: A, B, C, D, @, F, G;
        F: A, B, C, D, E, @, G;
        G: A, B, C, D, E, F, @;
    }
    OneOf8 {
        A: @, B, C, D, E, F, G, H;
        B: A, @, C, D, E, F, G, H;
        C: A, B, @, D, E, F, G, H;
        D: A, B, C, @, E, F, G, H;
        E: A, B, C, D, @, F, G, H;
        F: A, B, C, D, E, @, G, H;
        G: A, B, C, D, E, F, @, H;
        H: A, B, C, D, E, F, G, @;
    }
    OneOf9 {
        A: @, B, C, D, E, F, G, H, I;
        B: A, @, C, D, E, F, G, H, I;
        C: A, B, @, D, E, F, G, H, I;
        D: A, B, C, @, E, F, G, H, I;
        E: A, B, C, D, @, F, G, H, I;
        F: A, B, C, D, E, @, G, H, I;
        G: A, B, C, D, E, F, @, H, I;
        H: A, B, C, D, E, F, G, @, I;
        I: A, B, C, D, E, F, G, H, @;
    }
    OneOf10 {
        A: @, B, C, D, E, F, G, H, I, J;
        B: A, @, C, D, E, F, G, H, I, J;
        C: A, B, @, D, E, F, G, H, I, J;
        D: A, B, C, @, E, F, G, H, I, J;
        E: A, B, C, D, @, F, G, H, I, J;
        F: A, B, C, D, E, @, G, H, I, J;
        G: A, B, C, D, E, F, @, H, I, J;
        H: A, B, C, D, E, F, G, @, I, J;
        I: A, B, C, D, E, F, G, H, @, J;
        J: A, B, C, D, E, F, G, H, I, @;
    }
    OneOf11 {
        A: @, B, C, D, E, F, G, H, I, J, K;
        B: A, @, C, D, E, F, G, H, I, J, K;
        C: A, B, @, D, E, F, G, H, I, J, K;
        D: A, B, C, @, E, F, G, H, I, J, K;
        E: A, B, C, D, @, F, G, H, I, J, K;
        F: A, B, C, D, E, @, G, H, I, J, K;
        G: A, B, C, D, E, F, @, H, I, J, K;
        H: A, B, C, D, E, F, G, @, I, J, K;
        I: A, B, C, D, E, F, G, H, @, J, K;
        J: A, B, C, D, E, F, G, H, I, @, K;
        K: A, B, C, D, E, F, G, H, I, J, @;
    }
    OneOf12 {
        A: @, B, C, D, E, F, G, H, I, J, K, L;
        B: A, @, C, D, E, F, G, H, I, J, K, L;
        C: A, B, @, D, E, F, G, H, I, J, K, L;
        D: A, B, C, @, E, F, G, H, I, J, K, L;
        E: A, B, C, D, @, F, G, H, I, J, K, L;
        F: A, B, C, D, E, @, G, H, I, J, K, L;
        G: A, B, C, D, E, F, @, H, I, J, K, L;
        H: A, B, C, D, E, F, G, @, I, J, K, L;
        I: A, B, C, D, E, F, G, H, @, J, K, L;
        J: A, B, C, D, E, F, G, H, I, @, K, L;
        K: A, B, C, D, E, F, G, H, I, J, @, L;
        L: A, B, C, D, E, F, G, H, I, J, K, @;
    }
);

/// Represents a type that can be converted either [`From`] or [`TryInto`] the given types
///
/// Also conditionally implements [`Clone`], [`Copy`], [`Debug`](core::fmt::Debug), [`Display`], [`Eq`], [`Hash`](core::hash::Hash) and [`PartialEq`]
///
/// Accepts at least 2 types, up to 12 types
///
/// ## Examples
///
/// ### either `u32` or `char`
/// ```
/// # use one_of::one_of;
/// let x: one_of!(u32, char) = 42.into();
/// assert_eq!(Some(42u32), x.into());
/// assert_eq!(Option::<char>::None, x.into());
/// ```
///
/// ### some type of integer
/// ```
/// # use one_of::one_of;
/// let x: one_of!(i8, i16, i32, i64, u8, u16, u32, u64) = 42.into();
/// assert_eq!(Option::<i8>::None, x.into());
/// assert_eq!(Option::<i16>::None, x.into());
/// assert_eq!(Some(42i32), x.into());
/// assert_eq!(Option::<i64>::None, x.into());
/// assert_eq!(Option::<u8>::None, x.into());
/// assert_eq!(Option::<u16>::None, x.into());
/// assert_eq!(Option::<u32>::None, x.into());
/// assert_eq!(Option::<u64>::None, x.into());
/// ```
#[macro_export]
macro_rules! one_of {
    ($a:ty, $b:ty) => {
        $crate::OneOf2<$a, $b>
    };
    ($a:ty, $b:ty, $c:ty) => {
        $crate::OneOf3<$a, $b, $c>
    };
    ($a:ty, $b:ty, $c:ty, $d:ty) => {
        $crate::OneOf4<$a, $b, $c, $d>
    };
    ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty) => {
        $crate::OneOf5<$a, $b, $c, $d, $e>
    };
    ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty) => {
        $crate::OneOf6<$a, $b, $c, $d, $e, $f>
    };
    ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty) => {
        $crate::OneOf7<$a, $b, $c, $d, $e, $f, $g>
    };
    ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty) => {
        $crate::OneOf8<$a, $b, $c, $d, $e, $f, $g, $h>
    };
    ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty) => {
        $crate::OneOf9<$a, $b, $c, $d, $e, $f, $g, $h, $i>
    };
    ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty) => {
        $crate::OneOf10<$a, $b, $c, $d, $e, $f, $g, $h, $i, $j>
    };
    ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty) => {
        $crate::OneOf11<$a, $b, $c, $d, $e, $f, $g, $h, $i, $j, $k>
    };
    ($a:ty, $b:ty, $c:ty, $d:ty, $e:ty, $f:ty, $g:ty, $h:ty, $i:ty, $j:ty, $k:ty, $l:ty) => {
        $crate::OneOf12<$a, $b, $c, $d, $e, $f, $g, $h, $i, $j, $k, $l>
    };
}

macro_rules! gen_case {
    ($d:tt $($t:ty { $($v:ident $p:ident $g:ident $b:ident,)* })+) => {
        /// Pattern matching against [`one_of`] types, similar to `match`
        ///
        /// Groups of match arms are separated by `;`
        ///
        /// Unlike `match`, [`case`] only accepts blocks as values of match arms
        ///
        /// ## Examples
        ///
        /// ### matching against one of the integer types
        /// ```
        /// use one_of::{case, one_of};
        /// case!(<one_of!(i8, i64, u8, u64)>::from(42u64),
        ///     // i8
        ///     _ => {
        ///         panic!("not i8");
        ///     };
        ///
        ///     // i64
        ///     _ => {
        ///         panic!("not i64");
        ///     };
        ///
        ///     // u8
        ///     _ => {
        ///         panic!("not u8");
        ///     };
        ///
        ///     // u64
        ///     0 ..= 41 => {
        ///         panic!("not less than 42");
        ///     }
        ///     n => {
        ///         assert_eq!(n, 42);
        ///     };
        /// );
        /// ```
        ///
        /// ### gaurds are also supported, just like `match`
        /// ```
        /// # use one_of::{case, one_of};
        /// case!(<one_of!(bool, &str, i64)>::from("Hello, world!"),
        ///     // bool
        ///     _ => {
        ///         panic!("not bool");
        ///     };
        ///
        ///     // &str
        ///     s if s.starts_with("Hello, ") => {
        ///         assert_eq!(&s[7 ..], "world!");
        ///     }
        ///     _ => {
        ///         panic!("not other strings");
        ///     };
        ///
        ///     // i64
        ///     _ => {
        ///         panic!("not i64");
        ///     };
        /// );
        /// ```
        /// this is the equivalent using `match` and a custom enum
        /// ```
        /// enum BoolStrOrInt<'a> {
        ///     Bool(bool),
        ///     Str(&'a str),
        ///     Int(i64),
        /// }
        ///
        /// match BoolStrOrInt::Str("Hello, world!") {
        ///     // bool
        ///     BoolStrOrInt::Bool(_) => {
        ///         panic!("not bool");
        ///     }
        ///
        ///     // &str
        ///     BoolStrOrInt::Str(s) if s.starts_with("Hello, ") => {
        ///         assert_eq!(&s[7 ..], "world!");
        ///     }
        ///     BoolStrOrInt::Str(_) => {
        ///         panic!("not other strings");
        ///     }
        ///
        ///     // i64
        ///     BoolStrOrInt::Int(_) => {
        ///         panic!("not i64");
        ///     }
        /// }
        /// ```
        #[macro_export]
        macro_rules! case {
            $(
                ($d x:expr, $($d ($d $p:pat $d (if $d $g:expr)? => $d $b:block)+ ;)+) => {
                    match $d x {
                        $($d ($crate::$t::$v($d $p) $d (if $d $g)? => $d $b)+)+
                    }
                };
            )+
        }
    };
}

gen_case!($
    OneOf2 {
        A pa ga ba,
        B pb gb bb,
    }
    OneOf3 {
        A pa ga ba,
        B pb gb bb,
        C pc gc bc,
    }
    OneOf4 {
        A pa ga ba,
        B pb gb bb,
        C pc gc bc,
        D pd gd bd,
    }
    OneOf5 {
        A pa ga ba,
        B pb gb bb,
        C pc gc bc,
        D pd gd bd,
        E pe ge be,
    }
    OneOf6 {
        A pa ga ba,
        B pb gb bb,
        C pc gc bc,
        D pd gd bd,
        E pe ge be,
        F pf gf bf,
    }
    OneOf7 {
        A pa ga ba,
        B pb gb bb,
        C pc gc bc,
        D pd gd bd,
        E pe ge be,
        F pf gf bf,
        G pg gg bg,
    }
    OneOf8 {
        A pa ga ba,
        B pb gb bb,
        C pc gc bc,
        D pd gd bd,
        E pe ge be,
        F pf gf bf,
        G pg gg bg,
        H ph gh bh,
    }
    OneOf9 {
        A pa ga ba,
        B pb gb bb,
        C pc gc bc,
        D pd gd bd,
        E pe ge be,
        F pf gf bf,
        G pg gg bg,
        H ph gh bh,
        I pi gi bi,
    }
    OneOf10 {
        A pa ga ba,
        B pb gb bb,
        C pc gc bc,
        D pd gd bd,
        E pe ge be,
        F pf gf bf,
        G pg gg bg,
        H ph gh bh,
        I pi gi bi,
        J pj gj bj,
    }
    OneOf11 {
        A pa ga ba,
        B pb gb bb,
        C pc gc bc,
        D pd gd bd,
        E pe ge be,
        F pf gf bf,
        G pg gg bg,
        H ph gh bh,
        I pi gi bi,
        J pj gj bj,
        K pk gk bk,
    }
    OneOf12 {
        A pa ga ba,
        B pb gb bb,
        C pc gc bc,
        D pd gd bd,
        E pe ge be,
        F pf gf bf,
        G pg gg bg,
        H ph gh bh,
        I pi gi bi,
        J pj gj bj,
        K pk gk bk,
        L pl gl bl,
    }
);
