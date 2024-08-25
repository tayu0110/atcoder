use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, i64); m]}

    let mut dist = vec![vec![i64::MAX; n]; n];
    for (u, v, w) in e {
        dist[u - 1][v - 1] = w;
    }

    for i in 0..n {
        dist[i][i] = 0;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] < i64::MAX && dist[k][j] < i64::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][k].saturating_add(dist[k][j]));
                }
            }
        }
    }

    let mut dp = vec![vec![i64::MAX; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = 0;
    }
    for i in 0..1 << n {
        for j in 0..n {
            if i & (1 << j) != 0 && dp[i][j] < i64::MAX {
                for k in 0..n {
                    if dist[j][k] < i64::MAX {
                        let next = i | (1 << k);
                        dp[next][k] = dp[next][k].min(dp[i][j].saturating_add(dist[j][k]));
                    }
                }
            }
        }
    }

    let &res = dp[(1 << n) - 1].iter().min().unwrap();
    if res == i64::MAX {
        println!("No")
    } else {
        println!("{res}")
    }
}
