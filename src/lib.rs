extern crate proc_macro;
use std::ops::Range;

use oeis::OeisSeq;
use proc_macro::{TokenStream, TokenTree};

mod affine;
mod oeis;
use affine::AffineSeq;
use sequence::Sequence;

mod sequence;

struct Parser {
    tokens: Vec<TokenTree>,
    idx: usize,
}

impl Parser {
    fn new(token_stream: TokenStream) -> Self {
        Self {
            tokens: token_stream.into_iter().collect::<Vec<_>>(),
            idx: 0,
        }
    }

    fn peek(&self) -> Option<TokenTree> {
        self.tokens.get(self.idx).cloned()
    }

    fn step(&mut self) {
        self.idx += 1;
    }

    fn step_back(&mut self) {
        self.idx -= 1;
    }

    fn munch_integer(&mut self) -> Option<i128> {
        match self.peek() {
            Some(TokenTree::Literal(lit)) => {
                self.step();
                let int_str = lit.to_string();
                Some(
                    int_str
                        .parse()
                        .expect(&format!("unable to parse {:?} as integer", int_str)),
                )
            }
            _ => None,
        }
    }

    fn munch_punct(&mut self, ch: char) -> Option<()> {
        match self.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == ch => {
                self.step();
                Some(())
            }
            _ => None,
        }
    }

    fn munch_range(&mut self) -> Option<Range<i128>> {
        let start = self.munch_integer()?;
        println!("start is {:?}", start);
        self.munch_punct('.')?;
        println!("after dot");
        self.munch_punct('.')?;
        println!("after dot");

        let inclusive = self.munch_punct('=').is_some();

        let end = self.munch_integer()?;
        Some(start..(end + if inclusive { 1 } else { 0 }))
    }
}

#[proc_macro]
pub fn int_seq(token_stream: TokenStream) -> TokenStream {
    println!("token_stream is {:?}", token_stream);
    let mut parser = Parser::new(token_stream);
    let mut seq = Vec::new();

    // parse sequences of the form:
    //  - `1..128`
    //  - `1,2,4,8,16..128`
    //  - `1,2,4,8,16..=128`
    loop {
        // munch integer literal
        match parser.munch_integer() {
            Some(x) => seq.push(x),
            None => break,
        }

        // munch comma
        if parser.munch_punct(',').is_none() {
            parser.step_back();
            seq.pop();
            break;
        }
    }
    // munch range
    let range = parser.munch_range().expect("expected range");

    seq.push(range.start);
    let end = range.end;

    println!("inferring");

    let inferred_seq: Box<dyn Sequence> = match AffineSeq::infer(&seq) {
        Some(seq) => Box::new(seq),
        None => match OeisSeq::infer(&seq) {
            Some(seq) => Box::new(seq),
            None => panic!("could not infer sequence"),
        },
    };

    println!("inferred_seq is bruh");

    let generated_seq = inferred_seq.generate(&seq, end);
    println!("generated_seq is {:?}", generated_seq);

    format!("{:?}", generated_seq).parse().unwrap()
}
