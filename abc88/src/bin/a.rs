#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: usize};

    if a >= n % 500 {
        println!("Yes");
    } else {
        println!("No");
    }
}
