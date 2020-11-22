#![feature(negative_impls)]
#![feature(optin_builtin_traits)]
#![no_std]

// https://github.com/rust-lang/rust/issues/30905#issuecomment-173327799
#[doc(hidden)]
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

            $(
                impl<$($l,)* $v $(,$r)*> From<$v> for $n<$($l,)* $v $(,$r)*>
                    where
                        $(($v, $l): Different,)*
                        $(($v, $r): Different,)* {
                    fn from(x: $v) -> Self {
                        Self::$v(x)
                    }
                }

                impl<$($l,)* $v $(,$r)*> Into<Option<$v>> for $n<$($l,)* $v $(,$r)*>
                    where
                        $(($v, $l): Different,)*
                        $(($v, $r): Different,)* {
                    fn into(self) -> Option<$v> {
                        match self {
                            Self::$v(x) => Some(x),
                            _ => None,
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
