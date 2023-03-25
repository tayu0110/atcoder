#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: usize, b: usize};

    println!("{}", std::cmp::min(a*n, b));
}
