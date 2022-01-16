# Trait for IS_SIGNED associated constant

At the time of writing, separation of implementations for [primitive integers][primitive integer] depending on whether or not they are signed
is complicated because negative trait bounds and [impl specialization](https://rust-lang.github.io/rfcs/1210-impl-specialization.html) are
available only on Nightly Rust.

This crate alleviates the pain, though does not solve the problem entirely.

Excerpt from Rust's reference:

# Numeric types

## Integer types

The unsigned integer types consist of:

Type   | Minimum | Maximum
-------|---------|-------------------
`u8`   | 0       | 2<sup>8</sup>-1
`u16`  | 0       | 2<sup>16</sup>-1
`u32`  | 0       | 2<sup>32</sup>-1
`u64`  | 0       | 2<sup>64</sup>-1
`u128` | 0       | 2<sup>128</sup>-1

The signed two's complement integer types consist of:

Type   | Minimum            | Maximum
-------|--------------------|-------------------
`i8`   | -(2<sup>7</sup>)   | 2<sup>7</sup>-1
`i16`  | -(2<sup>15</sup>)  | 2<sup>15</sup>-1
`i32`  | -(2<sup>31</sup>)  | 2<sup>31</sup>-1
`i64`  | -(2<sup>63</sup>)  | 2<sup>63</sup>-1
`i128` | -(2<sup>127</sup>) | 2<sup>127</sup>-1

[primitive integer]: https://doc.rust-lang.org/stable/reference/types/numeric.html
[trait]: https://doc.rust-lang.org/book/ch10-02-traits.html
[associated constants]: https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>