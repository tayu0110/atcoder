use itertools::Itertools;
use proconio::*;

fn main() {
    input! {_: usize, k: usize, s: marker::Bytes}

    let set = s
        .windows(k)
        .sorted()
        .dedup_with_count()
        .max_set_by_key(|v| v.0);
    println!(
        "{}\n{}",
        set[0].0,
        set.iter()
            .map(|v| v.1.iter().copied().map(|v| v as char).collect::<String>())
            .join(" ")
    );
}
