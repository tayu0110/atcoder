use itertools::Itertools;
use proconio::*;

fn main() {
    input! {mut s: marker::Chars}
    s.sort_unstable();
    println!("{}", s.iter().join(""))
}
