use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {s: Chars}

    for (i, c) in s.into_iter().enumerate().rev() {
        if c == 'a' {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1")
}
