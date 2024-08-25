#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars}

    for i in (0..s.len()).filter(|v| v % 2 == 0) {
        print!("{}", s[i]);
    }
    println!();
}
