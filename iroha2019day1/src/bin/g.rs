use proconio::input;

fn main() {
    input! {n: usize, m: usize, k: usize, mut a: [i64; n]};
    let mut b = vec![0];
    b.append(&mut a);

    const INF: i64 = 0x3f3f3f3f3f3f3f3f;
    let mut dp = vec![vec![-INF; m+1]; n+1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..m {
            if dp[i][j] != -INF {
                for k in i+1..=std::cmp::min(i+k, n) {
                    dp[k][j+1] = std::cmp::max(dp[i][j] + b[k], dp[k][j+1]);
                }
            }
        }
    }

    let mut res = -1;
    for i in 0..k {
        res = std::cmp::max(res, dp[n-i][m]);
    }
    println!("{}", res);
}