use std::collections::VecDeque;

use proconio::*;

fn dfs(
    now: usize,
    par: usize,
    depth: usize,
    parlist: &mut [usize],
    rank: &mut [usize],
    unused: &mut Vec<usize>,
    t: &[Vec<usize>],
) {
    rank[now] = depth;
    for &to in &t[now] {
        if to != par {
            if parlist[to] == usize::MAX {
                parlist[to] = now;
                dfs(to, now, depth + 1, parlist, rank, unused, t);
            } else {
                unused.push(to);
                unused.push(now);
            }
        }
    }
}

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m], q: usize, query: [(usize, usize); q]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut rank = vec![0; n];
    let mut parlist = vec![usize::MAX; n];
    let mut unused = vec![];
    dfs(0, 0, 0, &mut parlist, &mut rank, &mut unused, &t);
    unused.sort_unstable();
    unused.dedup();

    let mut dist = vec![vec![usize::MAX; n]; unused.len()];
    for (i, &start) in unused.iter().enumerate() {
        let mut nt = VecDeque::from([start]);
        dist[i][start] = 0;
        while let Some(now) = nt.pop_front() {
            for &to in &t[now] {
                if dist[i][to] == usize::MAX {
                    dist[i][to] = dist[i][now] + 1;
                    nt.push_back(to);
                }
            }
        }
    }

    let mut doubling = vec![vec![usize::MAX; n]; 20];
    doubling[0] = parlist.clone();
    for i in 0..19 {
        for j in 0..n {
            if doubling[i][j] != usize::MAX {
                doubling[i + 1][j] = doubling[i][doubling[i][j]];
            }
        }
    }

    for (u, v) in query.into_iter().map(|(u, v)| (u - 1, v - 1)) {
        let mut res = 0;
        {
            let (mut u, mut v) = (u, v);
            if rank[u] < rank[v] {
                (u, v) = (v, u);
            }

            if rank[u] != rank[v] {
                let mut diff = rank[u] - rank[v];
                for i in (0..20).rev() {
                    if diff & (1 << i) != 0 {
                        diff -= 1 << i;
                        u = doubling[i][u];
                        res += 1 << i;
                    }
                }
            }

            if u != v {
                for i in (0..20).rev() {
                    if doubling[i][u] != doubling[i][v] {
                        u = doubling[i][u];
                        v = doubling[i][v];
                        res += 2 << i;
                    }
                }
                res += 2;
            }
        }

        for i in 0..unused.len() {
            for (j, &y) in unused.iter().enumerate() {
                res = res.min(dist[i][u] + dist[j][v] + dist[i][y]);
            }
        }

        println!("{res}")
    }
}
