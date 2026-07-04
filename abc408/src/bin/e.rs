use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut e: [(usize, usize, u32); m]}

    for bit in (0..30).rev() {
        let mut t = vec![vec![]; n];
        for &(u, v, w) in &e {
            t[u - 1].push((v - 1, (w >> bit) & 1));
            t[v - 1].push((u - 1, (w >> bit) & 1));
        }

        let mut dist = vec![u32::MAX; n];
        let mut nt = VecDeque::new();
        nt.push_back((0, 0));
        while let Some((now, cost)) = nt.pop_front() {
            if dist[now] < u32::MAX {
                continue;
            }
            dist[now] = cost;
            for &(to, w) in &t[now] {
                if dist[to] < u32::MAX {
                    continue;
                }

                if w == 1 {
                    nt.push_back((to, cost | w));
                } else {
                    nt.push_front((to, cost | w));
                }
            }
        }

        if dist[n - 1] == 0 {
            e.retain(|v| (v.2 >> bit) & 1 == 0);
        }
    }

    let mut t = vec![vec![]; n];
    for (u, v, w) in e {
        t[u - 1].push((v - 1, w));
        t[v - 1].push((u - 1, w));
    }

    let mut dist = vec![u32::MAX; n];
    dist[0] = 0;
    let mut nt = vec![0];
    while let Some(now) = nt.pop() {
        for &(to, w) in &t[now] {
            if dist[to] < u32::MAX {
                continue;
            }

            dist[to] = dist[now] | w;
            nt.push(to);
        }
    }

    println!("{}", dist[n - 1])
}
