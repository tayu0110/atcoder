use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, i32); m]}

    let mut t = vec![vec![]; n];
    for (a, b, c) in p {
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }

    let mut dp = vec![vec![-1; 1 << n]; n];
    for i in 0..n {
        dp[i][1 << i] = 0;
    }
    for j in 1..1 << n {
        for i in 0..n {
            if dp[i][j] < 0 {
                continue;
            }

            for &(to, c) in &t[i] {
                if j & (1 << to) != 0 {
                    continue;
                }

                let next = j | (1 << to);
                dp[to][next] = dp[to][next].max(dp[i][j] + c);
            }
        }
    }

    println!("{}", dp.iter().flatten().max().unwrap())
}
