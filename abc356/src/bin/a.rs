use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, l: usize, r: usize}

    let mut t = (1..=n).collect::<Vec<_>>();
    t[l - 1..r].reverse();
    println!("{}", t.iter().join(" "))
}
