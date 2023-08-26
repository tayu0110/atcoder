use proconio::*;

fn main() {
    input! {l: usize, r: usize, mut s: marker::Chars}
    s[l - 1..r].reverse();
    println!("{}", s.into_iter().collect::<String>())
}
