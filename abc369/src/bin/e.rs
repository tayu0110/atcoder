use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut e: [(usize, usize, usize); m], q: usize}
    e.iter_mut().for_each(|v| {
        v.0 -= 1;
        v.1 -= 1;
    });

    let mut dp = vec![vec![usize::MAX; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }
    for &(u, v, t) in &e {
        dp[u][v] = dp[u][v].min(t);
        dp[v][u] = dp[v][u].min(t);
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k].saturating_add(dp[k][j]));
            }
        }
    }

    for _ in 0..q {
        input! {k: usize, b: [usize; k]}

        let mut res = usize::MAX;
        for mut b in b.into_iter().permutations(k) {
            b.iter_mut().for_each(|b| *b -= 1);
            for i in 0..1 << k {
                let mut now = 0;
                let mut dist = 0;
                for (j, &b) in b.iter().enumerate() {
                    let (mut u, mut v, t) = e[b];
                    if i & (1 << j) != 0 {
                        (u, v) = (v, u);
                    }

                    dist += dp[now][u] + t;
                    now = v;
                }
                dist += dp[now][n - 1];
                res = res.min(dist);
            }
        }

        println!("{res}")
    }
}
