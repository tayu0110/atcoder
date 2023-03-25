#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars};

    println!("{}{}{}", s[0], s.len()-2, s.last().unwrap());
}
