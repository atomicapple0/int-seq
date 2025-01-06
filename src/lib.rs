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

    // if let Some(affine) = Affine::infer_from(&seq) {
    //     let seq = affine.generate(seq[0], end);
    //     return format!("{:?}", seq).parse().unwrap();
    // };

    // consult oeis for more complicated cases :D
    // we get a json from a url like this: `https://oeis.org/search?q=1,2,4,8&fmt=json`

    // construct url
    let url = format!(
        "https://oeis.org/search?q={:?}&fmt=json",
        seq.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    println!("url is {:?}", url);

    // get json
    let response = reqwest::blocking::get(url).unwrap();
    println!("response is {:?}", response);
    let body = response.text().unwrap();
    println!("response body is {:?}", body);

    // parse json
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    println!("--------------------------------");
    println!("--------------------------------");
    println!("--------------------------------");
    println!("--------------------------------");
    println!("json is {:?}", json);

    match json {
        serde_json::Value::Array(arr) => {
            for item in arr {
                println!("------");
                // println!("item is {:?}", item);
                match item {
                    serde_json::Value::Object(obj) => {
                        println!("obj keys are {:?}", obj.keys().collect::<Vec<_>>());
                        match &obj["data"] {
                            serde_json::Value::String(name) => {
                                println!("name is {:?}", name);
                            }
                            _ => panic!("expected string"),
                        }
                    }
                    _ => panic!("expected object"),
                }
            }
        }
        _ => panic!("expected array"),
    }

    todo!()
}
