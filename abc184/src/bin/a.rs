#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: i32, b: i32, c: i32, d: i32}

    println!("{}", a * d - b * c);
}