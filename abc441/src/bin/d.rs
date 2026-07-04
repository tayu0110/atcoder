use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, l: usize, s: usize, t: usize, e: [(usize, usize, usize); m]}

    let mut g = vec![vec![]; n];
    for (u, v, c) in e {
        g[u - 1].push((v - 1, c));
    }

    let mut buf = vec![(0, 0)];
    for _ in 0..l {
        let mut next = vec![];
        for (now, cost) in buf {
            for &(to, c) in &g[now] {
                if cost + c <= t {
                    next.push((to, cost + c));
                }
            }
        }

        next.sort_unstable();
        next.dedup();

        buf = next;
    }

    let mut ret = buf
        .into_iter()
        .filter_map(|(i, c)| (s..=t).contains(&c).then_some(i + 1))
        .collect::<Vec<_>>();
    ret.sort_unstable();
    ret.dedup();

    println!("{}", ret.into_iter().join(" "));
}
