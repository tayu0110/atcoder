#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

// #[fastout]
fn main() {
    const MOD: usize = 998244353;
    input! {n: usize, m: usize, k: usize, s: usize, t: usize, x: usize, p: [(usize, usize); m]}

    let mut graph = vec![vec![]; n];
    for (u, v) in p {
        graph[u-1].push(v-1);
        graph[v-1].push(u-1);
    }

    let mut map = std::collections::HashMap::new();
    map.insert((s-1, 0usize), 1usize);

    for _ in 0..k {
        let mut tmp = std::collections::HashMap::new();
        for (&(now, x_checksum), v) in &map {
            for to in &graph[now] {
                let mut nx = x_checksum;
                if *to == x-1 {
                    nx ^= 1;
                }

                let entry = tmp.entry((*to, nx)).or_insert(0);
                *entry += v;
                *entry %= MOD;
            }
        }

        std::mem::swap(&mut map, &mut tmp);
    }

    println!("{}", map.get(&(t-1, 0)).unwrap_or(&0));
}
