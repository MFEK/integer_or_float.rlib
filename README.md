# `integer_or_float` v0.3.2

<div align="center">
  <a href="https://docs.rs/integer_or_float">
    <img src="https://docs.rs/integer_or_float/badge.svg" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/integer_or_float">
    <img src="https://img.shields.io/crates/v/integer_or_float.svg" alt="Version">
  </a>
  <a href="https://github.com/MFEK/integer_or_float.rlib/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/integer_or_float.svg" alt="License">
  </a>
</div>

This is a Rust type that holds an integer or a float.

```rust
// from src/backing_types.rs
#[cfg(not(feature = "x64-backing-store"))]
pub type f_iof = f32;
#[cfg(not(feature = "x64-backing-store"))]
pub type i_iof = i32;

/// A generic container for an "integer or a float".
pub enum IntegerOrFloat {
    Integer(i_iof),
    Float(f_iof)
}
```

At first it was just a Rust implementation of the UFO datatype `integer or float`, which appears all over the UFO spec, but most importantly in the affine matrices used by glyph components.

Now (v0.1.4) it's generic. You can compile with the experimental feature `x64-backing-store` to get an `IntegerOrFloat` defined as such:

```rust
pub enum IntegerOrFloat {
    Integer(i64),
    Float(f64)
}
```

Rather than the default:

```rust
pub enum IntegerOrFloat {
    Integer(i32),
    Float(f32)
}
```

## Discretionary features
### `more-serde`
Enables serde with `serde/std`.

### `numerical-traits`
Enables `float-cmp` and `num-traits` crates, which provide `::ApproxEq` and `::{cast::ToPrimitive, Zero, One, NumCast}` respectively.

### `faster-strconv`
Enables [_Ryū_ (龍)](https://crates.io/crates/ryu), for faster float→string conversions.

### `default`
As of v0.2, an alias for `numerical-traits`. This is a breaking change from v0.1, serde no longer default!

### `fat`
As of v0.2, enables all.

# License

```plain
    Copyright 2022 Fredrick R. Brennan <copypaste@kittens.ph>

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
```
