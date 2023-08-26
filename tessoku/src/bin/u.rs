use proconio::*;

fn main() {
    input! {n: usize, mut p: [(usize, i64); n]}
    p.iter_mut().for_each(|(i, _)| *i -= 1);

    let mut dp = vec![vec![0; n + 1]; n + 1];
    for len in (1..=n).rev() {
        for i in 0..n {
            let j = i + len;
            if j > n {
                break;
            }

            let (s, t) = p[i];
            let (v, w) = p[j - 1];
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] + if i <= s && s < j { t } else { 0 });
            dp[i][j - 1] = dp[i][j - 1].max(dp[i][j] + if i <= v && v < j { w } else { 0 });
        }
    }

    println!("{}", (0..n).map(|i| dp[i][i]).max().unwrap())
}
