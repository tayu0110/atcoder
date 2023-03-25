#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: i32, a: i32};

    if x < a {
        println!("0");
    } else {
        println!("10");
    }
}
