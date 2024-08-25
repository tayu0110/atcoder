use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, q: usize, k: usize, a: [usize; k], e: [(usize, usize); m], q: [(usize, usize); q]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut dist = vec![vec![usize::MAX; n]; k];
    for (i, &a) in a.iter().enumerate() {
        let mut nt = VecDeque::new();
        nt.push_back(a - 1);
        dist[i][a - 1] = 0;
        while let Some(now) = nt.pop_front() {
            for &to in &t[now] {
                if dist[i][to] == usize::MAX {
                    nt.push_back(to);
                    dist[i][to] = dist[i][now] + 1;
                }
            }
        }
    }

    for (src, dst) in q {
        let mut res = usize::MAX;
        for i in 0..k {
            for (j, &t) in a.iter().enumerate() {
                res = res.min(dist[i][src - 1] + dist[i][t - 1] + dist[j][dst - 1]);
            }
        }

        println!("{res}")
    }
}
