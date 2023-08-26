use proconio::*;

fn main() {
    input! {n: usize, _: usize, a: [[usize; n]; n], q: usize, p: [(usize, usize); q]}

    let mut dp = a.clone();
    for i in 0..n {
        for j in 0..n {
            if dp[i][j] == 0 {
                dp[i][j] = std::usize::MAX >> 10;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
            }
        }
    }

    for (s, t) in p {
        let (s, t) = ((s - 1) % n, (t - 1) % n);

        if dp[s][t] == std::usize::MAX >> 10 {
            println!("-1")
        } else {
            println!("{}", dp[s][t])
        }
    }
}
