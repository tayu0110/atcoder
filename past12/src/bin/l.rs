use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [i64; n], c: [(i64, usize, usize, usize); m]}

    let mut bonus = vec![[[0; 3]; 3]; n + 2];
    for (p, q, l, r) in c {
        bonus[q][l][r] += p;
    }

    let mut res = 0;
    for s in 3..6 {
        let mut dp = vec![[-1; 3]; n + 1];
        dp[0][0] = 0;
        for (i, &a) in a.iter().enumerate() {
            for j in 0..3 {
                if dp[i][j] < 0 {
                    continue;
                }

                // not use
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] + bonus[i + 1][j][(s - j) % 3]);
                // use
                let next = (j + 1) % 3;
                dp[i + 1][next] = dp[i + 1][next].max(dp[i][j] + a + bonus[i + 1][j][(s - j) % 3]);
            }
        }

        res = res.max(dp[n][s % 3] + bonus[n + 1][s % 3][0]);
    }

    println!("{res}")
}
