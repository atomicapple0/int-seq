extern crate proc_macro;

use proc_macro::TokenStream;

mod affine;
mod oeis;
mod parser;
use parser::parse_int_seq;

mod sequence;

/// Given a sequence of integers that includes an range ellipsis, we deduce the
/// integers that are omitted using various heuristics. This macro will produce
/// an array of integers at compile time. We do not support lazy iterators yet
/// since some integer sequences do not have a simple closed form formula.
///
/// We currently use two heuristics for inferring integer sequences:
///  - Affine sequences are sequences of the form `a*i + b`
///  - OEIS sequences are sequences that are in the OEIS database
///
/// This feature is inspired by Raku lang's sequence operator (`...`).
///
/// Example:
/// ```rust
/// use int_seq::int_seq;
///
/// // affine sequence
/// assert_eq!(int_seq!(57, 64, 71, 78, 85..100), &[57, 64, 71, 78, 85, 92, 99]);
/// // inclusive upper bound
/// assert_eq!(int_seq!(3, 6..=12), &[3, 6, 9, 12]);
/// // basic range
/// assert_eq!(int_seq!(1..5), &[1, 2, 3, 4]);
/// // powers of 2
/// assert_eq!(int_seq!(1, 2, 4, 8, 16..=1024), &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]);
/// // McKay-Thompson series of class 32e for the Monster group (OEIS A082303).
/// assert_eq!(int_seq!(-4, -9, 4, 10..26), &[-4, -9, 4, 10, -4, -12, 6, 15, -7, -17, 7, 19, -8, -22, 10])
/// ```
#[proc_macro]
pub fn int_seq(token_stream: TokenStream) -> TokenStream {
    let (seq, end) = parse_int_seq(&token_stream).expect(&format!(
        "could not parse token stream. token stream: {:x?}",
        token_stream
    ));
    let inferred_seq = sequence::infer_sequence(&seq).expect("could not infer sequence");
    let generated_seq = inferred_seq.generate(&seq, end);
    format!("&{:?}", generated_seq.as_slice()).parse().unwrap()
}
