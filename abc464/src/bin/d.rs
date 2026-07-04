use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut ret = vec![];
    for _ in 0..t {
        input! {n: usize, s: marker::Bytes, x: [i64; n], y: [i64; n - 1]}

        // 0: sunny, 1: rainy
        let mut dp = [0; 2];
        if s[0] == b'S' {
            dp[1] = -x[0];
        } else {
            dp[0] = -x[0];
        }

        for i in 0..n - 1 {
            let mut new = [i64::MIN; 2];
            if s[i + 1] == b'S' {
                // sunny
                new[0] = dp[0];
                if dp[1] > i64::MIN {
                    new[0] = new[0].max(dp[1] + y[i]);
                }
                // rainy
                if dp[0] > i64::MIN {
                    new[1] = dp[0] - x[i + 1];
                }
                if dp[1] > i64::MIN {
                    new[1] = new[1].max(dp[1] - x[i + 1]);
                }
            } else {
                // sunny
                if dp[0] > i64::MIN {
                    new[0] = dp[0] - x[i + 1];
                }
                if dp[1] > i64::MIN {
                    new[0] = new[0].max(dp[1] - x[i + 1] + y[i]);
                }
                // rainy
                new[1] = dp[0].max(dp[1]);
            }
            dp = new;
        }
        ret.push(dp[0].max(dp[1]));
    }
    println!("{}", ret.iter().join("\n"))
}
