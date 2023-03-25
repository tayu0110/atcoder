#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: [usize; n]}

    println!("{}", a.into_iter().fold(0, |s, v| s + v - 1));
}
