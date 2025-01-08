# int-seq

[![GHA Status]][GitHub Actions] [![Latest Version]][crates.io] [![Documentation]][docs.rs] ![License]

A Rust macro for generating integer sequences. This is inspired by [Raku lang's sequence operator (`...`)](https://doc.perl6.org/language/operators#infix_...).

Here are some examples of how the `...` operator works in Raku taken from a [blog post](https://buttondown.com/hillelwayne/archive/raku-a-language-for-gremlins/).

```raku
> 0,1,2...10
(0 1 2 3 4 5 6 7 8 9 10)
> 0,2,4...10
(0 2 4 6 8 10)
> 1,2,4...10
(1 2 4 8)
```

Clearly, this is a extremely powerful and useful feature that is missing in Rust.

---

This crate fills this need by providing the `int_seq!` macro.

Given a sequence of integers that includes an range ellipsis, we deduce the
integers that are omitted using various heuristics. This macro will produce
an array of integers at compile time. We do not support lazy iterators yet
since some integer sequences do not have a simple closed form formula.

We currently use two heuristics for inferring integer sequences:

- Affine sequences are of the form `a*i + b`
- OEIS sequences are in the [On-Line Encyclopedia of Integer Sequences (OEIS)](https://oeis.org/) database. We use `reqwest` to perform HTTP requests within a procedural macro to query the database. By doing this at compile time, we can avoid runtime overheads

The same examples from Raku but written with the `int_seq!` macro:

```rust
use int_seq::int_seq;

assert_eq!(int_seq!(0, 1, 2..=10), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
assert_eq!(int_seq!(0, 2, 4..=10), [0, 2, 4, 6, 8, 10]);
assert_eq!(int_seq!(1, 2, 4..=10), [1, 2, 4, 8]);
```

Some more complex examples:

```rust
use int_seq::int_seq;

// affine sequence
assert_eq!(int_seq!(57, 64, 71, 78, 85..100), [57, 64, 71, 78, 85, 92, 99]);
// powers of 2
assert_eq!(int_seq!(1, 2, 4, 8, 16..=1024), [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]);
// fibonacci sequence
assert_eq!(int_seq!(0, 1, 1, 2, 3, 5, 8, 13..100), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
// McKay-Thompson series of class 32e for the Monster group (OEIS A082303).
assert_eq!(int_seq!(-4, -9, 4, 10..26), [-4, -9, 4, 10, -4, -12, 6, 15, -7, -17, 7, 19, -8, -22, 10])
```

## Limitations

Does not yet support decreasing sequences. e.g. `int_seq!(10, 9, 8..=0)` does not work yet.

[GitHub Actions]: https://github.com/atomicapple0/int-seq/actions
[GHA Status]: https://github.com/atomicapple0/int-seq/actions/workflows/rust.yml/badge.svg
[crates.io]: https://crates.io/crates/int-seq
[Latest Version]: https://img.shields.io/crates/v/int-seq.svg
[Documentation]: https://docs.rs/int-seq/badge.svg
[docs.rs]: https://docs.rs/int-seq
[License]: https://img.shields.io/crates/l/int-seq.svg
