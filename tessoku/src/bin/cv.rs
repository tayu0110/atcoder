use proconio::*;

fn main() {
    input! {n: usize, p: [(f32, f32); n]}

    let mut dp = vec![vec![f32::MAX; 1 << n]; n];
    dp[0][0] = 0.0;
    for i in 0..1 << n {
        for j in 0..n {
            if dp[j][i] == f32::MAX {
                continue;
            }

            let (x, y) = p[j];

            for k in 0..n {
                if i & (1 << k) != 0 {
                    continue;
                }

                let next = i | (1 << k);
                dp[k][next] = dp[k][next].min(dp[j][i] + (x - p[k].0).hypot(y - p[k].1));
            }
        }
    }

    println!("{}", dp[0][(1 << n) - 1])
}
