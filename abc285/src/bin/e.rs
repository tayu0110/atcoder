use proconio::input;

fn main() {
    input! {n: usize, a: [i64; n]}

    let mut memo = vec![0; n + 1];
    for i in 1..=n {
        for j in 1..=i {
            memo[i] += a[std::cmp::min(i + 1 - j, j) - 1];
        }
    }

    let mut dp = vec![vec![std::i64::MIN; n + 1]; n + 1];
    dp[1][0] = 0;
    for i in 1..n {
        for j in 0..n {
            dp[i + 1][j + 1] = std::cmp::max(dp[i + 1][j + 1], dp[i][j]);
            dp[i + 1][0] = std::cmp::max(dp[i + 1][0], dp[i][j] + memo[j]);
        }
    }

    let mut res = 0;
    for i in 0..n {
        res = std::cmp::max(res, dp[n][i] + memo[i]);
    }

    println!("{}", res);
}
