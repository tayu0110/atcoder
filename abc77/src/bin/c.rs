use std::ops::Add;

use proconio::*;

fn main() {
    input! {n: usize, mut a: [u32; n], mut b: [u32; n], mut c: [u32; n]}

    a.sort_unstable();
    c.sort_unstable();

    println!(
        "{}",
        b.into_iter()
            .map(|b| a.partition_point(|&a| a < b) * (n - c.partition_point(|&c| c <= b)))
            .reduce(Add::add)
            .unwrap()
    )
}
