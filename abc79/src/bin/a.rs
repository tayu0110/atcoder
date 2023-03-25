#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars}

    if s[0] == s[1] && s[1] == s[2] {
        println!("Yes");
    } else if s[1] == s[2] && s[2] == s[3] {
        println!("Yes");
    } else {
        println!("No");
    }
}
