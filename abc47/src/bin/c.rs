use proconio::{input, marker::Chars};

fn main() {
    input! {mut s: Chars}
    s.dedup();
    println!("{}", s.len() - 1);
}
