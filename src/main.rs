mod affine;

use affine::Affine;

fn main() {
    let affine = Affine::infer_from(&[101, 106, 111]).unwrap();
    println!("{:?}", affine.generate(101, 996));
}
