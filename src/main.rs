use int_seq::int_seq;

fn main() {
    let x = int_seq!(-4, -9, 4, 10..26);
    println!("{:?}", x);
}
