use flow::maximum_matching_of_bipartite_graph;
use proconio::*;

fn main() {
    input! {n: usize, c: [marker::Bytes; n]}

    println!(
        "{}",
        maximum_matching_of_bipartite_graph(
            n * 2,
            c.into_iter()
                .enumerate()
                .map(|(i, v)| v
                    .into_iter()
                    .enumerate()
                    .filter_map(move |(j, c)| (c == b'#').then_some((i, j + n))))
                .flatten()
                .collect()
        )
        .len()
    )
}
