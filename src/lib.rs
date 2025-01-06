extern crate proc_macro;
use std::ops::Range;

use proc_macro::{TokenStream, TokenTree};

mod affine;
use affine::Affine;

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

    fn munch_integer(&mut self) -> Option<i32> {
        match self.peek() {
            Some(TokenTree::Literal(lit)) => {
                self.step();
                Some(lit.to_string().parse().unwrap())
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

    fn munch_range(&mut self) -> Option<Range<i32>> {
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
    //  - `e..g`
    //  - `a,b,c,d,e..g`
    //  - `a,b,c,d,e..=g`
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

    let affine = Affine::infer_from(&seq).unwrap();
    let seq = affine.generate(seq[0], end);

    format!("{:?}", seq).parse().unwrap()
}
