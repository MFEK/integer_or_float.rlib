# `integer_or_float` v0.1.5

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

