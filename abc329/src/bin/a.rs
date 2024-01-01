use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: marker::Chars}
    println!("{}", s.iter().join(" "))
}
