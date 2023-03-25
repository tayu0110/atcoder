#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut s: Chars, t: Chars}

    s.append(&mut s.clone());

    for i in 0..t.len() {
        let mut bad = false;
        for j in i..i+t.len() {
            if s[j] != t[j-i] {
                bad = true;
            }
        }

        if !bad {
            println!("Yes");
            std::process::exit(0);
        }
    }

    println!("No");
}
