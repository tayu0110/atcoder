use itertools::Itertools;
use proconio::*;

fn main() {
    input! {k: i32, x: i32}

    println!("{}", (x - k + 1..=x + k - 1).join(" "))
}
