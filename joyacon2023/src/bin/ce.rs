use proconio::input;

fn main() {
    input! {n: usize, k: usize, d: usize, a:[i64; n]}

    let mut dp = vec![vec![-1; d]; k + 1];
    dp[0][0] = 0;
    for a in a {
        for i in (0..k).rev() {
            for j in 0..d {
                if dp[i][j] < 0 {
                    continue;
                }

                dp[i + 1][(j + a as usize) % d] =
                    std::cmp::max(dp[i + 1][(j + a as usize) % d], dp[i][j] + a);
            }
        }
    }

    println!("{}", dp[k][0]);
}
