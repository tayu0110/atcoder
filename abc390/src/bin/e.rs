use proconio::*;

fn main() {
    input! {n: usize, x: usize, food: [(usize, i64, usize); n]}

    let mut s = vec![vec![]; 3];
    for (v, a, c) in food {
        s[v - 1].push((a, c));
    }

    let mut memo = vec![];
    for s in s {
        let mut dp = vec![-1; x + 1];
        dp[0] = 0;
        for (a, c) in s {
            for i in (0..x + 1).rev() {
                if dp[i] < 0 {
                    continue;
                }
                if i + c > x {
                    continue;
                }
                dp[i + c] = dp[i + c].max(dp[i] + a);
            }
        }
        for i in 0..x {
            dp[i + 1] = dp[i + 1].max(dp[i])
        }
        memo.push(dp);
    }

    let (mut l, mut r) = (0, i64::MAX >> 10);
    while r - l > 1 {
        let min = (r + l) / 2;
        let mut c = 0;
        for memo in &memo {
            let pos = memo.partition_point(|&v| v < min);
            c += pos;
        }

        if c > x {
            r = min;
        } else {
            l = min;
        }
    }

    println!("{l}")
}
