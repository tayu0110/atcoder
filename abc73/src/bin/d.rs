use proconio::input;

fn main() {
    input! {n: usize, m: usize, r: usize, r: [usize; r], p: [(usize, usize, usize); m]}
    const INF: usize = std::usize::MAX >> 10;

    let mut dp = vec![vec![INF; n]; n];
    for (a, b, c) in p {
        dp[a - 1][b - 1] = c;
        dp[b - 1][a - 1] = c;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = std::cmp::min(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }

    let n = r.len();
    let mut t = vec![vec![]; n];
    for i in 0..n {
        for j in i + 1..n {
            let (l, r) = (r[i] - 1, r[j] - 1);
            t[i].push((j, dp[l][r]));
            t[j].push((i, dp[r][l]));
        }
    }

    let mut dp = vec![vec![INF; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = 0;
    }

    for i in 1..1 << n {
        for j in 0..n {
            if i & (1 << j) == 0 {
                continue;
            }

            for &(to, w) in &t[j] {
                if i & (1 << to) != 0 {
                    continue;
                }

                dp[i | (1 << to)][to] = std::cmp::min(dp[i | (1 << to)][to], dp[i][j] + w);
            }
        }
    }

    println!("{}", dp[(1 << n) - 1].iter().min().unwrap())
}
