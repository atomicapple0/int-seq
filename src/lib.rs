extern crate proc_macro;

mod affine;
mod oeis;
mod parser;
mod sequence;

use parser::parse_int_seq;
use proc_macro::TokenStream;

/// Run doctests from the README.md file
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;

/// Given a sequence of integers that includes an range ellipsis, we deduce the
/// integers that are omitted using various heuristics. This macro will produce
/// an array of integers at compile time. We do not support lazy iterators yet
/// since some integer sequences do not have a simple closed form formula.
///
/// We currently use two heuristics for inferring integer sequences:
///  - Affine sequences are of the form `a*i + b`
///  - OEIS sequences are in the [On-Line Encyclopedia of Integer Sequences (OEIS)](https://oeis.org/) database. We use `reqwest` to perform HTTP requests within a procedural macro to query the database. By doing this at compile time, we can avoid runtime overheads
///
/// This is inspired by [Raku lang's sequence operator (`...`)](https://doc.perl6.org/language/operators#infix_...).
///
/// Example:
/// ```rust
/// use int_seq::int_seq;
///
/// // fibonacci sequence
/// assert_eq!(int_seq!(0, 1, 1, 2, 3, 5, 8, 13..100), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
/// ```
#[proc_macro]
pub fn int_seq(token_stream: TokenStream) -> TokenStream {
    let (seq, end) = parse_int_seq(&token_stream).unwrap_or_else(|| {
        panic!(
            "could not parse token stream. token stream: {:x?}",
            token_stream
        )
    });
    let inferred_seq = sequence::infer_sequence(&seq).expect("could not infer sequence");
    let generated_seq = inferred_seq.generate(&seq, end);
    let comma_seperated_ints = generated_seq
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    format!("[{}]", comma_seperated_ints).parse().unwrap()
}
