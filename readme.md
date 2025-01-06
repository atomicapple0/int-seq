# int-seq

A Rust macro for generating integer sequences. This is inspired by [Raku lang's sequence operator (`...`)](https://doc.perl6.org/language/operators#infix_...).

Given a sequence of integers that includes an range ellipsis, we deduce the
integers that are omitted using various heuristics. This macro will produce
an array of integers at compile time. We do not support lazy iterators yet
since some integer sequences do not have a simple closed form formula.

We currently use two heuristics for inferring integer sequences:
- Affine sequences are of the form `a*i + b`
- OEIS sequences are in the [On-Line Encyclopedia of Integer Sequences (OEIS)](https://oeis.org/) database. We use `reqwest` to perform HTTP requests within a procedural macro to query the database. By doing this at compile time, we can avoid runtime overheads

```rust
use int_seq::int_seq;

// affine sequence
assert_eq!(int_seq!(57, 64, 71, 78, 85..100), &[57, 64, 71, 78, 85, 92, 99]);
// inclusive upper bound
assert_eq!(int_seq!(3, 6..=12), &[3, 6, 9, 12]);
// basic range
assert_eq!(int_seq!(1..5), &[1, 2, 3, 4]);
// powers of 2
assert_eq!(int_seq!(1, 2, 4, 8, 16..=1024), &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]);
// fibonacci sequence
assert_eq!(int_seq!(0, 1, 1, 2, 3, 5, 8, 13..100), &[0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
// McKay-Thompson series of class 32e for the Monster group (OEIS A082303).
assert_eq!(int_seq!(-4, -9, 4, 10..26), &[-4, -9, 4, 10, -4, -12, 6, 15, -7, -17, 7, 19, -8, -22, 10])
```
