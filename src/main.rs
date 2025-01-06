mod affine;
use affine::Affine;
use int_seq::int_seq;

use core::ops::Range;

fn main() {
    let x = int_seq!(50, 55, 60, 65, 70..100);
    println!("{:?}", x);
}
