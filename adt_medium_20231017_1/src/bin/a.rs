use proconio::*;

fn main() {
    input! {mut s: marker::Chars}
    s.sort_unstable();
    println!("{}", s.into_iter().collect::<String>())
}
