[![crates.io](https://img.shields.io/crates/v/typenum.svg)](https://crates.io/crates/typenum)
[![Build Status](https://travis-ci.org/paholg/typenum.svg?branch=master)](https://travis-ci.org/paholg/typenum)

Typenum
=====

Typenum is a Rust library for type-level numbers evaluated at compile time. It currently
supports bits, unsigned integers, and signed integers.

For the full documentation, go [here](http://paholg.com/typenum).

Here is a short example of its use:

```rust
use std::ops::Add;
use typenum::consts::{N2, P3, P4};
use typenum::int::Integer;
use typenum::Pow;

type X = <P3 as Add<P4>>::Output;
assert_eq!(<X as Integer>::to_i32(), 7);

type Y = <N2 as Pow<P3>>::Output;
assert_eq!(<Y as Integer>::to_i32(), -8);
```
