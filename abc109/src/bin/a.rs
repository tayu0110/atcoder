#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize}

    if a % 2 == 0 || b % 2 == 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
