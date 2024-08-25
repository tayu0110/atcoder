use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: marker::Chars}
    println!(
        "{}",
        s.into_iter()
            .dedup_with_count()
            .map(|(i, c)| format!("{c} {i}"))
            .join(" ")
    )
}
