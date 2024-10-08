use flow::hopcroft_karp;
use proconio::*;

fn main() {
    input! {n: usize, c: [marker::Bytes; n]}

    println!(
        "{}",
        hopcroft_karp(
            &c.into_iter()
                .enumerate()
                .flat_map(|(i, v)| v
                    .into_iter()
                    .enumerate()
                    .filter_map(move |(j, c)| (c == b'#').then_some((i, j + n))))
                .collect::<Vec<_>>()
        )
        .len()
    )
}
