#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: i32}

    println!("{}", std::cmp::max(0, x));
}
