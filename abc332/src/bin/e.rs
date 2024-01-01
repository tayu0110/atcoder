use proconio::*;

fn main() {
    input! {n: usize, d: usize, w: [usize; n]}

    let mut memo = vec![0; 1 << n];
    for i in 0..1 << n {
        let mut sum = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                sum += w[j];
            }
        }
        memo[i] = sum;
    }

    let ave = w.iter().map(|&w| w as f64).sum::<f64>() / d as f64;
    let mut dp = vec![vec![f64::MAX; d + 1]; 1 << n];
    dp[0][0] = 0.0;
    for i in 0..(1 << n) - 1 {
        for j in 0..d {
            if dp[i][j] == f64::MAX {
                continue;
            }

            let mask = ((1 << n) - 1) ^ i;
            let tr = mask.trailing_zeros();
            dp[i | (1 << tr)][j + 1] = dp[i | (1 << tr)][j + 1]
                .min(dp[i][j] + (w[tr as usize] as f64 - ave) * (w[tr as usize] as f64 - ave));
            let mask = mask ^ (1 << tr);
            let mut rem = mask;
            while rem > 0 {
                let p = rem | (1 << tr);
                let sum = memo[p];

                dp[i | p][j + 1] =
                    dp[i | p][j + 1].min(dp[i][j] + (sum as f64 - ave) * (sum as f64 - ave));

                rem = (rem - 1) & mask;
            }
        }
    }

    let mut res = f64::MAX;
    for i in 0..=d {
        let mut t = dp[(1 << n) - 1][i];
        let zero = d - i;
        t += ave * ave * zero as f64;
        res = res.min(t);
    }

    println!("{}", res / d as f64)
}
