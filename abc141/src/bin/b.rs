#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars};

    for(i, c) in s.into_iter().enumerate() {
        if i % 2 == 0 {
            if c == 'L' {
                println!("No");
                std::process::exit(0);
            }
        } else if c == 'R' {
            println!("No");
            std::process::exit(0);
        }
    }

    println!("Yes");
}
