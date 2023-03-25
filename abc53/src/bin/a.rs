#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: usize};

    if x < 1200 {
        println!("ABC");
    } else {
        println!("ARC");
    }
}
