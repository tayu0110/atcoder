#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize}
    println!("{}", (a-1) * (b-1));
}
