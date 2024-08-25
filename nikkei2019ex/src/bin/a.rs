use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: String}
    println!("{}", (1..=s.len()).join("\n"))
}
