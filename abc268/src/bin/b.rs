#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars, t: Chars};

    if s.len() > t.len() {
        println!("No");
        std::process::exit(0);
    }

    for (i, v) in s.into_iter().enumerate() {
        if v != t[i] {
            println!("No");
            std::process::exit(0);
        }
    }

    println!("Yes");
}
