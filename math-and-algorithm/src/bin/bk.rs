use itertools::Itertools;
use proconio::*;
use std::collections::VecDeque;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut min = vec![0; n + 1];
    let mut max = vec![120; n + 1];
    max[1] = 0;

    let mut t = vec![vec![]; n + 1];
    for (a, b) in e {
        t[a].push(b);
        t[b].push(a);
    }

    let mut nt = VecDeque::new();
    nt.push_back(1);
    let mut checked = vec![false; n + 1];

    while let Some(now) = nt.pop_front() {
        checked[now] = true;

        for &to in &t[now] {
            if !checked[to] {
                nt.push_back(to);
            }

            let s = min[now];
            let l = (max[now] + 1).min(120);

            min[to] = min[to].max(s);
            max[to] = max[to].min(l);
        }
    }

    println!("{}", max[1..].iter().join("\n"))
}
