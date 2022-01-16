#![no_std]
#![doc = include_str!("../README.md")]

/// Trait ensuring that there is a constant boolean value that tells whether the type is signed
pub trait IsSigned {
    const IS_SIGNED: bool;
}

macro_rules! impl_for_unsigned {
    ($t:ty) => {
        impl IsSigned for $t {
            const IS_SIGNED: bool = false;
        }
    };
}

macro_rules! impl_for_signed {
    ($t:ty) => {
        impl IsSigned for $t {
            const IS_SIGNED: bool = true;
        }
    };
}

// https://doc.rust-lang.org/stable/reference/types/numeric.html

macro_rules! call_for_prim_unsigned_ints {
    ($macro:ident) => {
        $macro!(u8);
        $macro!(u16);
        $macro!(u32);
        $macro!(u64);
        $macro!(u128);
        $macro!(usize);
    };
}

macro_rules! call_for_prim_signed_ints {
    ($macro:ident) => {
        $macro!(i8);
        $macro!(i16);
        $macro!(i32);
        $macro!(i64);
        $macro!(i128);
        $macro!(isize);
    };
}

call_for_prim_unsigned_ints!(impl_for_unsigned);
call_for_prim_signed_ints!(impl_for_signed);