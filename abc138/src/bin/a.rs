#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, s: String};

    if a < 3200 {
        println!("red");
    } else {
        println!("{}", s);
    }
}
