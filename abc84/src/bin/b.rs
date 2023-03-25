#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, _b: usize, s: Chars}

    for (i, c) in s.into_iter().enumerate() {
        if i == a {
            if c != '-' {
                println!("No");
                std::process::exit(0);
            }
        } else {
            if !c.is_ascii_digit() {
                println!("No");
                std::process::exit(0);
            }
        }
    }

    println!("Yes");
}
