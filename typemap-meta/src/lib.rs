#![no_std]

//! A simple compile-time derive macro to create type-to-value maps.
//!
//! This approach in contrast to crates such as [`typemap`](https://crates.io/crates/typemap/)
//! or [`type-map`](https://crates.io/crates/type-map/) that perform run-time lookup.
//! The static typing brings compile-time safety and faster execution at the expense
//! of using a derive macro and generics.
//!
//! The crate is `no_std` compatible.
//!
//! # Example
//! ```
//! # use typemap_meta::*;
//! #[derive(Typemap)]
//! struct Test(i32, f32);
//!
//! let t = Test(1, 2.0);
//! assert_eq!(*get!(&t, i32), 1);
//! assert_eq!(*get!(&t, f32), 2.0);
//! ```

pub use typemap_meta_derive::*;

/// Helper trait to get a specific type `T` from a tuple struct containing disjoint heterogeneous types
pub trait Get<T> {
    fn get(&self) -> &T;
}

/// Convenience macro to get a specific type `$t` from a tuple struct `$s` containing disjoint heterogeneous types
#[macro_export]
macro_rules! get{
    ($s:expr, $t:ty) => {
        $crate::Get::<$t>::get($s)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Get, get};

    #[test]
    fn impl_get() {
        struct Test(i32, f32);
        impl Get<i32> for Test {
            fn get(&self) -> &i32 {
                &self.0
            }
        }
        impl Get<f32> for Test {
            fn get(&self) -> &f32 {
                &self.1
            }
        }
        let t = Test(1, 2.0);
        assert_eq!(*get!(&t, i32), 1);
        assert_eq!(*get!(&t, f32), 2.0);
    }

    #[test]
    fn derive_ok() {
        extern crate std;
        use std::marker::PhantomData;
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        struct A<T> {
            _f: PhantomData<T>
        }
        #[derive(crate::Typemap)]
        struct Test(i32, f32, A<u32>);
        let a = A { _f: PhantomData };
        let t = Test(1, 2.0, a);
        assert_eq!(*get!(&t, i32), 1);
        assert_eq!(*get!(&t, f32), 2.0);
        assert_eq!(*get!(&t, A<u32>), a);
    }
}
