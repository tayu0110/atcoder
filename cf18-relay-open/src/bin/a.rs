use itertools::Itertools;
use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {n: usize, s: [String; n]};

    let map = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let rev = map
        .iter()
        .copied()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect::<HashMap<&str, usize>>();

    println!(
        "{}",
        s.into_iter()
            .map(|s| {
                let idx = rev.get(s.as_str()).unwrap();
                map[(idx + 1) % 7]
            })
            .join("\n")
    )
}
