#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, d: [usize; n]};

    println!("{}", d.iter().enumerate().fold(0, |s, (i, v)| s + v * d.iter().skip(i+1).sum::<usize>()))
}
