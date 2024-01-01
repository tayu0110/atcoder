use proconio::*;

fn main() {
    input! {x: char, s: marker::Chars}

    println!("{}", s.into_iter().filter(|&c| c != x).collect::<String>())
}
