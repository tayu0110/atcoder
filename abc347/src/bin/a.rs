use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}
    println!(
        "{}",
        a.into_iter()
            .filter_map(|a| (a % k == 0).then_some(a / k))
            .join(" ")
    )
}
