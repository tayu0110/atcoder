#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v)in p {
        t[u-1].push(v-1);
        t[v-1].push(u-1);
    }

    let mut dp = vec![vec![std::usize::MAX; n]; 1 << n];

    let mut nt = std::collections::BinaryHeap::new();
    for i in 0..n {
        nt.push(std::cmp::Reverse((0, i, 0)));
    }

    while let Some(std::cmp::Reverse((nd, now, s))) = nt.pop() {
        if dp[s][now] < nd {
            continue;
        }
        dp[s][now] = nd;

        for to in &t[now] {
            let ns = s ^ (1 << *to);
            if dp[ns][*to] > nd + 1 {
                dp[ns][*to] = nd + 1;
                nt.push(std::cmp::Reverse((nd+1, *to, ns)));
            }
        }
    }

    println!("{}", dp.iter().fold(0usize, |s, v| s + *v.iter().min().unwrap()));
}
