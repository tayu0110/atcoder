use proconio::*;

fn main() {
    input! {s: marker::Chars, t: marker::Chars}
    println!("{}", s.into_iter().zip(t).filter(|(s, t)| s == t).count())
}
