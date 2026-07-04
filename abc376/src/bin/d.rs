use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    let mut rt = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        rt[b - 1].push(a - 1);
    }

    let mut forward = vec![usize::MAX; n];
    let mut backward = vec![usize::MAX; n];
    for (t, dist) in [(&t, &mut forward), (&rt, &mut backward)] {
        dist[0] = 0;
        let mut nt = VecDeque::new();
        nt.push_back(0);
        while let Some(now) = nt.pop_front() {
            for &to in &t[now] {
                if dist[to] != usize::MAX {
                    continue;
                }

                dist[to] = dist[now] + 1;
                nt.push_back(to);
            }
        }
    }

    // eprintln!("forward: {forward:?}, backward: {backward:?}");

    let mut res = usize::MAX;
    for i in 1..n {
        res = res.min(forward[i].saturating_add(backward[i]));
    }

    println!("{}", res as i64);
}
