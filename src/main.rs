mod affine;
use affine::Affine;

use core::ops::Range;

fn int_seq(x0: i32, x1: i32, seq: Range<i32>) -> Vec<i32> {
    let affine = Affine::infer_from(&[x0, x1, seq.start]).unwrap();
    affine.generate(x0, seq.end)
}

fn main() {
    let affine = Affine::infer_from(&[101, 106, 111]).unwrap();
    println!("{:?}", affine.generate(101, 996));
    println!("{:?}", int_seq(101, 106, 111..996));
}
