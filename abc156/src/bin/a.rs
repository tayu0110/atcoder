#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, r: usize};

    if n < 10 {
        println!("{}", r + 100 * (10 - n));
    } else {
        println!("{}", r);
    }
}
