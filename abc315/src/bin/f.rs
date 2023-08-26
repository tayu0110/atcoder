use proconio::*;

fn main() {
    input! {n: usize, p: [(f64, f64); n]}

    // distMAX = 1.4 * 10^8, so bad then 2^30 penalty
    const MAX_SKIP: usize = 31;
    let mut dp = vec![vec![f64::MAX; MAX_SKIP]; n];
    dp[0][0] = 0.0;

    for i in 0..n {
        let (x, y) = p[i];
        for skip in 0..(i + 1).min(MAX_SKIP) {
            for j in i + 1..n {
                let (nx, ny) = p[j];
                let d = (nx - x).hypot(ny - y);
                let next = skip + j - i - 1;
                if next >= MAX_SKIP {
                    break;
                }
                if dp[j][next] > dp[i][skip] + d {
                    dp[j][next] = dp[i][skip] + d;
                }
            }
        }
    }

    let mut res = f64::MAX;
    for i in 0..MAX_SKIP {
        if dp[n - 1][i] < f64::MAX {
            let r = if i == 0 {
                dp[n - 1][i]
            } else {
                dp[n - 1][i] + (1 << (i - 1)) as f64
            };

            if res > r {
                res = r;
            }
        }
    }

    println!("{}", res)
}
