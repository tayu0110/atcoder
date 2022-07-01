use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n*2]};
    const INF: usize = 111222333444555666;

    let n = 2 * n;
    let mut dp = vec![vec![INF; n+1]; n+1];

    for i in 0..n+1 {
        dp[i][i] = 0;
    }

    for len in 0..=n {
        if len % 2 != 0 {
            continue;
        }
        for i in 0..n {
            let j = i + len;
            if j > n {
                continue;
            }
            for k in i..j {
                let tmp = std::cmp::min(dp[i][k] + dp[k][j], dp[i][j]);
                dp[i][j] = tmp;
            }
            if i > 0 && j < n {
                let tmp = std::cmp::min(dp[i-1][j+1], dp[i][j] + std::cmp::max(a[i-1], a[j]) - std::cmp::min(a[i-1], a[j]));
                dp[i-1][j+1] = tmp;
            }
        }
    }

    println!("{}", dp[0][n]);
}