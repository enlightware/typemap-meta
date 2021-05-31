# typemap-meta

An [Enlightware® software](https://enlightware.ch).

## Overview

A simple compile-time (hence the meta) derive macro to create type-to-value maps (hence the typemap).
This approach in contrast to crates such as [`typemap`](https://crates.io/crates/typemap/) or [`type-map`](https://crates.io/crates/type-map/) that perform run-time lookup.
The static typing brings compile-time safety and faster execution at the expense of using a derive macro and generics.

The crate is `no_std` compatible.

## Usage

To use this crate, first add this to your `Cargo.toml`:

```toml
[dependencies]
typemap-meta = "0.1"
```

Then, you can create a tuple struct containing disjoint heterogeneous types, and derive `Typemap`, and then use the `get!` macro (a syntactic sugar around `Typemap::Get`):

```rust
#[derive(Typemap)]
struct Test(i32, f32);
let t = Test(1, 2.0);
assert_eq!(*get!(&t, i32), 1);
assert_eq!(*get!(&t, f32), 2.0);
```

## Crate structure

As currently procedural macros must be defined [in their own crate](https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro), we have two crates `typemap-meta` and `typemap-meta-derive`, the former re-exporting the macro from the later.
Only the former needs to be imported in your project.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](typemap-meta/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](typemap-meta/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.