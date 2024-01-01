use itertools::Itertools;
use proconio::*;

fn main() {
    input! {_: usize, s: marker::Bytes}
    println!("{}", if s.iter().all_equal() { "Yes" } else { "No" })
}
