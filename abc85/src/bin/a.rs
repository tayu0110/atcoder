#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut s: Chars}

    s[3] = '8';

    let s = s.into_iter().collect::<String>();
    println!("{}", s);
}
