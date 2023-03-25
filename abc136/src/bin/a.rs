#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: i32, b: i32, c: i32};

    println!("{}", std::cmp::max(0, c - (a - b)));
}
