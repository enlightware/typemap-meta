# typemap-meta

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Build Status][ci-badge]][ci-url]

[crates-badge]: https://img.shields.io/crates/v/typemap-meta
[crates-url]: https://crates.io/crates/typemap-meta
[docs-badge]: https://img.shields.io/docsrs/typemap-meta
[docs-url]: https://docs.rs/typemap-meta
[ci-badge]: https://github.com/enlightware/typemap-meta/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/enlightware/typemap-meta/actions

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
typemap-meta = "0.2"
```

Then, you can create a tuple struct containing disjoint heterogeneous types, and derive `Typemap`, and then use the `get!` macro (a syntactic sugar around `Typemap::Get`):

```rust
#[derive(Typemap)]
struct Test(i32, f32);
let t = Test(1, 2.0);
assert_eq!(*get!(t, i32), 1);
assert_eq!(*get!(t, f32), 2.0);
```

A mutable version is also available:

```rust
#[derive(Typemap)]
#[typemap_mut]
struct Test(i32, f32);
let mut t = Test(1, 2.0);
*get_mut!(t, i32) = 3;
*get_mut!(t, f32) = 4.0;
assert_eq!(*get!(t, i32), 3);
assert_eq!(*get!(t, f32), 4.0);
```

## Crate structure

As currently procedural macros must be defined [in their own crate](https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro), we have two crates `typemap-meta` and `typemap-meta-derive`, the former re-exporting the macro from the later.
Only the former needs to be imported in your project.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.