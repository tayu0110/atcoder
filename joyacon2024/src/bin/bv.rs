use proconio::*;

fn main() {
    input! {s: marker::Chars}

    println!("{}{}{}", s[0], s.len() - 2, s.last().unwrap())
}
