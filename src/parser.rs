use core::ops::Range;

use proc_macro::{TokenStream, TokenTree};

pub struct Parser {
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
        let is_negative = self.munch_punct('-').is_some();
        let parity = if is_negative { -1 } else { 1 };

        match self.peek() {
            Some(TokenTree::Literal(lit)) => {
                self.step();
                let int_str = lit.to_string();
                let num: i128 = int_str
                    .parse()
                    .expect(&format!("unable to parse {:?} as integer", int_str));
                Some(num * parity)
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
        self.munch_punct('.')?;
        self.munch_punct('.')?;

        let inclusive = self.munch_punct('=').is_some();

        let end = self.munch_integer()?;
        Some(start..(end + if inclusive { 1 } else { 0 }))
    }
}

/// Parse ast of the form:
///  - `1..128`
///  - `1,2,4,8,16..128`
///  - `1,2,4,8,16..=128`
pub(crate) fn parse_int_seq(token_stream: &TokenStream) -> Option<(Vec<i128>, i128)> {
    let mut parser = Parser::new(token_stream.clone());
    let mut seq = Vec::new();

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
    let range = parser.munch_range()?;

    seq.push(range.start);
    let end = range.end;
    Some((seq, end))
}
