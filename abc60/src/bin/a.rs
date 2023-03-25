#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: Chars, b: Chars, c: Chars}

    if a.last().unwrap() == &b[0] && b.last().unwrap() == &c[0] {
        println!("YES");
    } else {
        println!("NO");
    }
}
