use proconio::*;

const N: usize = 16;

fn main() {
    input! {n: usize, x: [usize; n]}

    let mut dp = vec![std::f64::MAX; 1 << N];
    dp[0] = 0.0;
    for i in 1..1 << N {
        for j in 1..N - 1 {
            let mut cnt = 0;
            let mut p = 0.0;
            for k in [j - 1, j, j + 1] {
                if i & (1 << k) == 0 {
                    cnt += 1;
                } else {
                    let prev = i ^ (1 << k);
                    p += dp[prev];
                }
            }
            if cnt == 3 {
                continue;
            }

            p = p / 3.0 + 1.0;
            if cnt > 0 {
                p = p * 3.0 / (3 - cnt) as f64;
            }

            if dp[i] > p {
                dp[i] = p;
            }
        }
    }

    let tar = x.into_iter().fold(0, |s, v| s | (1 << v));
    println!("{}", dp[tar])
}
