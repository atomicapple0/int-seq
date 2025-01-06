use int_seq::int_seq;

fn main() {
    let x = int_seq!(2, 4, 8, 16..=1024);
    println!("{:?}", x);
}
