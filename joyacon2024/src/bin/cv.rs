use std::cmp::Reverse;

use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    a.sort_unstable_by_key(|a| Reverse(*a));
    println!(
        "{}",
        a.into_iter()
            .enumerate()
            .filter_map(|(i, a)| (i % 2 == 0).then_some(a))
            .sum::<usize>()
    )
}
