#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {h: usize, w: usize, y: usize, x: usize};

    println!("{}", (h-y) * (w-x));
}
