use proconio::input;

fn main() {
    input! {n: usize, a: [[i64; n]; n]}

    let mut dp = vec![0; 1 << n];
    for i in 1..1 << n {
        let w = (0..n)
            .filter(|v| i & (1usize << v) != 0)
            .collect::<Vec<_>>();
        for &v in &w {
            for &w in &w {
                if v < w {
                    dp[i] += a[v][w];
                }
            }
        }
    }

    for i in 1..1 << n {
        let mut now = i;
        while now > 0 {
            let other = now ^ i;
            dp[i] = std::cmp::max(dp[i], dp[now] + dp[other]);
            now = (now - 1) & i;
        }
    }

    println!("{}", dp[(1 << n) - 1])
}
