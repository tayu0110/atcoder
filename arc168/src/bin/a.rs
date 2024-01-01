use itertools::Itertools;
use proconio::*;

fn main() {
    input! {_: usize, s: marker::Bytes}
    println!(
        "{}",
        s.iter()
            .dedup_with_count()
            .filter_map(|(c, b)| (b == &b'>').then_some(c * (c + 1) / 2))
            .sum::<usize>()
    );
}
