use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: String}
    let n = s.len();
    println!("{}", s[1..n - 1].split('|').map(|s| s.len()).join(" "))
}
