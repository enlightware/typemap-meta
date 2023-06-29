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
//! assert_eq!(*get!(t, i32), 1);
//! assert_eq!(*get!(t, f32), 2.0);
//! ```

pub use typemap_meta_derive::*;

/// Helper trait to get a specific type `T` from a tuple struct containing disjoint heterogeneous types
pub trait Get<T> {
    fn get(&self) -> &T;
}

/// Helper trait to mutably get a specific type `T` from a tuple struct containing disjoint heterogeneous types
pub trait GetMut<T> {
    fn get_mut(&mut self) -> &mut T;
}

/// Convenience macro to get a specific type `$t` from a tuple struct `$s` containing disjoint heterogeneous types
///
/// Passing a value is fine, as [`get`] will add a reference to `$t` before calling [`Get`].
#[macro_export]
macro_rules! get {
    ($s:expr, $t:ty) => {
        $crate::Get::<$t>::get(&$s)
    };
}

/// Convenience macro to mutably get a specific type `$t` from a tuple struct `$s` containing disjoint heterogeneous types
///
/// Passing a value is fine, as [`get_mut`] will add a reference to `$t` before calling [`GetMut`].
#[macro_export]
macro_rules! get_mut {
    ($s:expr, $t:ty) => {
        $crate::GetMut::<$t>::get_mut(&mut $s)
    };
}

#[cfg(test)]
mod tests {
    use crate::{get, Get, GetMut};

    // without using the generation macro

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
        assert_eq!(*get!(t, i32), 1);
        assert_eq!(*get!(t, f32), 2.0);
    }

    #[test]
    fn impl_get_mut() {
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

        impl GetMut<i32> for Test {
            fn get_mut(&mut self) -> &mut i32 {
                &mut self.0
            }
        }
        impl GetMut<f32> for Test {
            fn get_mut(&mut self) -> &mut f32 {
                &mut self.1
            }
        }

        let t = Test(1, 2.0);
        assert_eq!(*get!(t, i32), 1);
        assert_eq!(*get!(t, f32), 2.0);

        let mut t = t;

        *get_mut!(t, i32) = 3;
        *get_mut!(t, f32) = 4.0;
        assert_eq!(*get!(t, i32), 3);
        assert_eq!(*get!(t, f32), 4.0);
    }

    #[test]
    fn impl_get_ref() {
        struct Test<'a>(&'a i32, &'a f32);
        impl<'a> Get<&'a i32> for Test<'a> {
            fn get(&self) -> &&'a i32 {
                &self.0
            }
        }
        impl<'a> Get<&'a f32> for Test<'a> {
            fn get(&self) -> &&'a f32 {
                &self.1
            }
        }
        let t = Test(&1, &2.0);
        assert_eq!(**get!(t, &i32), 1);
        assert_eq!(**get!(t, &f32), 2.0);
    }

    #[test]
    fn impl_get_mut_ref() {
        struct Test<'a>(&'a i32, &'a f32);
        impl<'a> Get<&'a i32> for Test<'a> {
            fn get(&self) -> &&'a i32 {
                &self.0
            }
        }
        impl<'a> Get<&'a f32> for Test<'a> {
            fn get(&self) -> &&'a f32 {
                &self.1
            }
        }

        impl<'a> GetMut<&'a i32> for Test<'a> {
            fn get_mut(&mut self) -> &mut &'a i32 {
                &mut self.0
            }
        }
        impl<'a> GetMut<&'a f32> for Test<'a> {
            fn get_mut(&mut self) -> &mut &'a f32 {
                &mut self.1
            }
        }
        let t = Test(&1, &2.0);
        assert_eq!(**get!(t, &i32), 1);
        assert_eq!(**get!(t, &f32), 2.0);

        let mut t = t;

        *get_mut!(t, &i32) = &3;
        *get_mut!(t, &f32) = &4.0;
        assert_eq!(**get!(t, &i32), 3);
        assert_eq!(**get!(t, &f32), 4.0);
    }

    // with using the generation macro

    #[test]
    fn derive_scalar() {
        extern crate std;
        use std::marker::PhantomData;
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        struct A<T> {
            _f: PhantomData<T>,
        }
        #[derive(crate::Typemap)]
        struct Test(i32, f32, A<u32>);
        let a = A { _f: PhantomData };
        let t = Test(1, 2.0, a);
        assert_eq!(*get!(t, i32), 1);
        assert_eq!(*get!(t, f32), 2.0);
        assert_eq!(*get!(t, A<u32>), a);
    }

    #[test]
    fn derive_scalar_mut() {
        extern crate std;
        #[derive(Debug, PartialEq, Eq, Clone, Copy)]
        struct A<T>(T);
        #[derive(crate::Typemap)]
        #[typemap_mut]
        struct Test(i32, f32, A<u32>);
        let a = A(3);
        let t = Test(1, 2.0, a);
        assert_eq!(*get!(t, i32), 1);
        assert_eq!(*get!(t, f32), 2.0);
        assert_eq!(*get!(t, A<u32>), a);

        let mut t = t;
        let b = A(6);

        *get_mut!(t, i32) = 4;
        *get_mut!(t, f32) = 5.0;
        *get_mut!(t, A<u32>) = b;

        assert_eq!(*get!(t, i32), 4);
        assert_eq!(*get!(t, f32), 5.0);
        assert_eq!(*get!(t, A<u32>), b);
    }

    #[test]
    fn derive_struct() {
        #[derive(Debug, PartialEq)]
        struct A {}
        #[derive(Debug, PartialEq)]
        struct B {}
        #[derive(crate::Typemap)]
        struct Test(A, B);
        let t = Test(A {}, B {});
        assert_eq!(*get!(t, A), A {});
    }

    #[test]
    fn derive_struct_mut() {
        #[derive(Debug, PartialEq)]
        struct A { x: u32 }
        #[derive(Debug, PartialEq)]
        struct B { }
        #[derive(crate::Typemap)]
        #[typemap_mut]
        struct Test(A, B);
        let t = Test(A { x: 1 }, B {});
        assert_eq!(*get!(t, A), A { x: 1 });

        let mut t = t;

        get_mut!(t, A).x = 2;
        assert_eq!(*get!(t, A), A { x: 2 });
    }

    #[test]
    fn derive_ref() {
        #[derive(Debug, PartialEq)]
        struct A {}
        #[derive(Debug, PartialEq)]
        struct B {}
        #[derive(crate::Typemap)]
        struct Test<'l>(&'l A, &'l B, i32, f32);
        let a = A {};
        let b = B {};
        let t = Test(&a, &b, 1, 2.0);
        assert_eq!(**get!(t, &A), A {});
        assert_eq!(**get!(t, &B), B {});
        assert_eq!(*get!(t, i32), 1);
        assert_eq!(*get!(t, f32), 2.0);
    }

    #[test]
    fn derive_ref_mut() {
        #[derive(Debug, PartialEq)]
        struct A { x: u32 }
        #[derive(Debug, PartialEq)]
        struct B {}
        #[derive(crate::Typemap)]
        #[typemap_mut]
        struct Test<'l>(&'l A, &'l B, i32, f32);
        let a = A { x: 1 };
        let b = B {};
        let t = Test(&a, &b, 1, 2.0);
        assert_eq!(**get!(t, &A), A { x: 1 });
        assert_eq!(**get!(t, &B), B {});
        assert_eq!(*get!(t, i32), 1);
        assert_eq!(*get!(t, f32), 2.0);

        let mut t = t;

        *get_mut!(t, &A) = &A { x: 2 };
        *get_mut!(t, &B) = &B {};
        *get_mut!(t, i32) = 3;
        *get_mut!(t, f32) = 4.0;
        assert_eq!(**get!(t, &A), A { x: 2 });
        assert_eq!(**get!(t, &B), B {});
        assert_eq!(*get!(t, i32), 3);
        assert_eq!(*get!(t, f32), 4.0);
    }

    #[test]
    fn derive_box_trait() {
        extern crate std;
        use std::{boxed::Box, fmt::Debug};
        // trait TA and struct A
        trait TA {
            fn value_a(&self) -> i32;
        }
        #[derive(Debug, PartialEq)]
        struct A {
            v: i32,
        }
        impl TA for A {
            fn value_a(&self) -> i32 {
                self.v
            }
        }
        // trait TB and struct B
        trait TB {
            fn value_b(&self) -> f32;
        }
        #[derive(Debug, PartialEq)]
        struct B {
            v: f32,
        }
        impl TB for B {
            fn value_b(&self) -> f32 {
                self.v
            }
        }
        // instance and asserts
        #[derive(crate::Typemap)]
        struct Test(Box<dyn TA>, Box<dyn TB>);
        let t = Test(Box::new(A { v: 1 }), Box::new(B { v: 2.0 }));
        assert_eq!(get!(t, Box<dyn TA>).value_a(), 1);
        assert_eq!(get!(t, Box<dyn TB>).value_b(), 2.0);
    }

    #[test]
    fn derive_box_trait_mut() {
        extern crate std;
        use std::{boxed::Box, fmt::Debug};
        // trait TA and struct A
        trait TA {
            fn value_a(&self) -> i32;
            fn set_a(&mut self, a: i32);
        }
        #[derive(Debug, PartialEq)]
        struct A {
            v: i32,
        }
        impl TA for A {
            fn value_a(&self) -> i32 {
                self.v
            }
            fn set_a(&mut self, a: i32) {
                self.v = a;
            }
        }
        // trait TB and struct B
        trait TB {
            fn value_b(&self) -> f32;
            fn set_b(&mut self, b: f32);
        }
        #[derive(Debug, PartialEq)]
        struct B {
            v: f32,
        }
        impl TB for B {
            fn value_b(&self) -> f32 {
                self.v
            }
            fn set_b(&mut self, b: f32) {
                self.v = b;
            }
        }
        // instance and asserts
        #[derive(crate::Typemap)]
        #[typemap_mut]
        struct Test(Box<dyn TA>, Box<dyn TB>);
        let t = Test(Box::new(A { v: 1 }), Box::new(B { v: 2.0 }));
        assert_eq!(get!(t, Box<dyn TA>).value_a(), 1);
        assert_eq!(get!(t, Box<dyn TB>).value_b(), 2.0);

        let mut t = t;
        get_mut!(t, Box<dyn TA>).set_a(3);
        get_mut!(t, Box<dyn TB>).set_b(4.0);
        assert_eq!(get!(t, Box<dyn TA>).value_a(), 3);
        assert_eq!(get!(t, Box<dyn TB>).value_b(), 4.0);
    }

    #[test]
    fn derive_static_trait() {
        extern crate std;
        use std::fmt::Debug;
        // trait TA and struct A
        trait TA {
            fn value_a(&self) -> i32;
        }
        #[derive(Debug, PartialEq)]
        struct A {
            v: i32,
        }
        impl TA for A {
            fn value_a(&self) -> i32 {
                self.v
            }
        }
        // trait TB and struct B
        trait TB {
            fn value_b(&self) -> f32;
        }
        #[derive(Debug, PartialEq)]
        struct B {
            v: f32,
        }
        impl TB for B {
            fn value_b(&self) -> f32 {
                self.v
            }
        }
        // instance and asserts
        #[derive(crate::Typemap)]
        struct Test(&'static dyn TA, &'static dyn TB);
        let a: &'static dyn TA = &A { v: 1 };
        let b: &'static dyn TB = &B { v: 2.0 };
        let t = Test(a, b);
        assert_eq!(get!(t, &'static dyn TA).value_a(), 1);
        assert_eq!(get!(t, &'static dyn TB).value_b(), 2.0);
    }

    #[test]
    fn derive_static_trait_mut() {
        extern crate std;
        use std::fmt::Debug;
        // trait TA and struct A
        trait TA {
            fn value_a(&self) -> i32;
        }
        #[derive(Debug, PartialEq)]
        struct A {
            v: i32,
        }
        impl TA for A {
            fn value_a(&self) -> i32 {
                self.v
            }
        }
        // trait TB and struct B
        trait TB {
            fn value_b(&self) -> f32;
        }
        #[derive(Debug, PartialEq)]
        struct B {
            v: f32,
        }
        impl TB for B {
            fn value_b(&self) -> f32 {
                self.v
            }
        }
        // instance and asserts
        #[derive(crate::Typemap)]
        #[typemap_mut]
        struct Test(&'static dyn TA, &'static dyn TB);
        let a: &'static dyn TA = &A { v: 1 };
        let b: &'static dyn TB = &B { v: 2.0 };
        let t = Test(a, b);
        assert_eq!(get!(t, &'static dyn TA).value_a(), 1);
        assert_eq!(get!(t, &'static dyn TB).value_b(), 2.0);

        let mut t = t;
        let c: &'static dyn TA = &A { v: 3 };
        let d: &'static dyn TB = &B { v: 4.0 };

        *get_mut!(t, &'static dyn TA) = c;
        *get_mut!(t, &'static dyn TB) = d;

        assert_eq!(get!(t, &'static dyn TA).value_a(), 3);
        assert_eq!(get!(t, &'static dyn TB).value_b(), 4.0);
    }
}
