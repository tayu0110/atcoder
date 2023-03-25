#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize};

    if a < 10 && b < 10 {
        println!("{}", a * b);
    } else {
        println!("-1");
    }
}
