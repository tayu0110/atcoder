#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {h: usize, w: usize, map: [Chars; h]}

    let mut dist = vec![vec![std::usize::MAX; w]; h];
    let mut nt = std::collections::BinaryHeap::new();
    nt.push((1, 0, 0));

    while let Some((nd, r, c)) = nt.pop() {
        if dist[r][c] != std::usize::MAX {
            continue;
        }
        dist[r][c] = nd;

        if r + 1 < h && map[r + 1][c] != '#' && dist[r + 1][c] == std::usize::MAX {
            nt.push((nd + 1, r + 1, c));
        }
        if c + 1 < w && map[r][c + 1] != '#' && dist[r][c + 1] == std::usize::MAX {
            nt.push((nd + 1, r, c + 1));
        }
    }

    let res = dist
        .iter()
        .flatten()
        .filter(|&&c| c != std::usize::MAX)
        .max()
        .unwrap();
    println!("{}", res)
}
