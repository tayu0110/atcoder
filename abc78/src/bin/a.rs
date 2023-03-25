#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: char, y: char}

    if x == y {
        println!("=");
    } else if x < y {
        println!("<");
    } else {
        println!(">");
    }
}
