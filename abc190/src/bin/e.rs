#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m], k: usize, c: [usize; k]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
    }

    let mut dist = vec![vec![std::usize::MAX; k]; k];
    for (i, from) in c.iter().enumerate() {
        let from = *from - 1;

        let mut d = vec![std::usize::MAX; n];
        d[from] = 0;
        let mut nt = std::collections::VecDeque::new();
        nt.push_back(from);
        while let Some(now) = nt.pop_front() {
            for to in &t[now] {
                if d[*to] == std::usize::MAX {
                    d[*to] = d[now] + 1;
                    nt.push_back(*to);
                }
            }
        }

        for (j, v) in c.iter().enumerate() {
            dist[i][j] = d[*v-1];
        }
    }

    let mut dp = vec![vec![std::usize::MAX; 1 << k]; k];
    for i in 0..k {
        dp[i][1 << i] = 1;
    }

    for i in 0..(1 << k) {
        for now in 0..k {
            if dp[now][i] != std::usize::MAX {
                for to in 0..k {
                    if dist[now][to] != std::usize::MAX {
                        let nt = i | 1 << to;
                        dp[to][nt] = std::cmp::min(dp[to][nt], dp[now][i] + dist[now][to]);
                    }
                }
            }
        }
    }

    let mut res = std::usize::MAX;
    for i in 0..k {
        res = std::cmp::min(res, dp[i][(1 << k) - 1]);
    }

    if res == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
