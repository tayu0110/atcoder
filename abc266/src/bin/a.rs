#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars};

    let n = s.len();
    println!("{}", s[n / 2]);
}
