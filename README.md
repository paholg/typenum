[![crates.io](https://img.shields.io/crates/v/typenum.svg)](https://crates.io/crates/typenum)
[![Build Status](https://github.com/paholg/typenum/actions/workflows/check.yml/badge.svg)](https://github.com/paholg/typenum/actions/workflows/check.yml)

Typenum
=====

Typenum is a Rust library for type-level numbers evaluated at compile time. It
currently supports bits, unsigned integers, and signed integers.

Typenum depends only on libcore, and so is suitable for use on any platform!

For the full documentation, go [here](https://docs.rs/typenum).

### Importing

While `typenum` is divided into several modules, they are all re-exported
through the crate root, so you can import anything contained herein with `use
typenum::whatever;`, ignoring the crate structure.

You may also find it useful to treat the `consts` module as a prelude,
performing a glob import.

### Example

Here is a trivial example of `typenum`'s use:

```rust
use typenum::{Sum, Exp, Integer, N2, P3, P4};

type X = Sum<P3, P4>;
assert_eq!(<X as Integer>::to_i32(), 7);

type Y = Exp<N2, P3>;
assert_eq!(<Y as Integer>::to_i32(), -8);
```


And how to use constraints:

```rust
use typenum::{self, IsGreater, True, U0};
trait NonZero {
    type NonZero: IsGreater<U0, Output = True>;
}
```
<details>

  <summary>Unfold here to see a (somwhat contrived) exploitation of type-level integer arithmetic:</summary>
  
  ```rust
  // Imports to make rust playground happy
  use core::ops::{Div, Add, Mul};
  use typenum::{op, *};
  use generic_array::{GenericArray, ArrayLength};

  // Demo-time. A simple "flatten a 2d bool-array to a 1d byte-array"
  // Let's encapsulate a 2d array, similar to `[[bool; WIDTH]; HEIGHT]`
  pub struct FlattenDemo<
      Width: ArrayLength,
      Height: ArrayLength,
  > {
      unflattened: GenericArray<GenericArray<bool, Width >, Height>,
  }
  
  // For fun and profit, let's wrap-up the behavior in a trait.
  trait Flatten {
      // NOTE: the `generic-array` crate is pretty cool.
      type FlattenedLen: ArrayLength;
      fn flattened(self) -> GenericArray<bool, Self::FlattenedLen>;
  }
  
  
  // So here is the fun part: Flattening using compile-time maths, but using the type-system.
  // Flattening into a byte-array, you must ensure a multiple of 8, to Illustrate:
  // A: 2 x 3 = 6. so need a 1-byte array
  // B: 2 x 4 = 8: 8 is the nearest round-up, again; 1-byte array
  // C: 3 x 7 = 21: need to round up to 24: 3-byte array
  //
  // This is done by taking the needed bits, adding 7, then integer-division to 8
  // A: (6 + 7) / 8 = 13 / 8 = 1
  // B: (8 + 7) / 8 = 15 / 8 = 1
  // A: (21 + 7) / 8 = 28 / 8 = 3
  impl<Width: ArrayLength, Height: ArrayLength> Flatten for FlattenDemo<Width, Height> 
  where
      // Types are not values: Unlike integer values, all of which implement the behavior of
      // integers (multiplication, addition, division, etc) as an inherent part of the language,
      // the compiler has no way of knowing if a given type implements a given operation unless 
      // you explicitly specify...
      // 
      // 1. We must specify that the `Width` type must implement behavior in which it `Mul`tiplies
      //    the `Height` type:
      Width: Mul<Height>,
      // 2. We also must specify that this `Mul`tiply behaviors `Output` implements the 
      //    "`Add`ition on `U7`" behavior:
      op!(Width * Height): Add<U7>,
      // 3. And so on: This is the constraint "The result of (width * height) + 7 must implement
      //    division by 8", but without the `op!` convenience macro:
      <<Width as Mul<Height>>::Output as Add<U7>>::Output: Div<U8>,
      // With the convenience macro:
      // op!((Width * Height) + U7): Div<U8>,
      op!((Width * Height + U7) / U8): ArrayLength,
  
  {
      // Et, voila! Through the power of the type system, we have lifted arithmetic to compile 
      // time, without the use of nightly, the need to think about machine-representation of 
      // values (usize/u64/etc).
      type FlattenedLen = op!((Width * Height + U7) / U8);
      fn flattened(self) -> GenericArray<bool, Self::FlattenedLen> {
          todo!()
      }
  }
  ```
</details>

For more non-trivial examples, with real-world use, check the full list of
reverse dependencies [here](https://crates.io/crates/typenum/reverse_dependencies). Of note are
[dimensioned](https://crates.io/crates/dimensioned/) which does compile-time
type checking for arbitrary unit systems and
[generic-array](https://crates.io/crates/generic-array/) which provides arrays
whose length you can generically refer to.


### Error messages


Typenum's error messages aren't great, and can be difficult to parse. The good
news is that the fine folks at Auxon have written a tool to help with it. Please
take a look at [tnfilt](https://github.com/auxoncorp/tnfilt).

### License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
