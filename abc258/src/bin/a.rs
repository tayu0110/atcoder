#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {k: usize};

    println!("{}:{:02}", 21 + k / 60, k % 60);
}
