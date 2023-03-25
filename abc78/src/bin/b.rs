#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: usize, y: usize, z: usize}

    if x / (y + z) * (y + z) + z == x {
        println!("{}", x / (y + z));
    } else {
        println!("{}", x / (y + z) - 1);
    }
}
