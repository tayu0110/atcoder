#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: usize}

    println!("{}", (x / 100 + 1) * 100 - x);
}
