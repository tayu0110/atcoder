use itertools::Itertools;
use proconio::*;

fn solve(s: &[u8], upper: usize) -> usize {
    if upper == 0 {
        return 0;
    }
    let r = upper
        .to_string()
        .bytes()
        .map(|s| s - b'0')
        .collect::<Vec<_>>();
    // dp[is_equal][still_leading_zeros][used_r][used_s]
    let mut dp = vec![vec![vec![vec![0usize; s.len() + 1]; r.len() + 1]; 2]; 2];
    dp[1][1][0][0] = 1;
    for i in 0..r.len() {
        for j in 0..=s.len() {
            for k in 0..2 {
                for l in 0..2 {
                    for next in 0..10 {
                        if l == 1 && r[i] < next {
                            continue;
                        }

                        // is_equal
                        let nl = (l == 1 && r[i] == next) as usize;
                        // still_leading_zeros
                        let nk = (k == 1 && next == 0) as usize;
                        if j == 0 {
                            dp[nl][nk][i + 1][j] += dp[l][k][i][j];
                        }
                        if j < s.len() && next == s[j] && nk == 0 {
                            dp[nl][nk][i + 1][j + 1] += dp[l][k][i][j];
                        }
                        if j == s.len() {
                            dp[nl][nk][i + 1][j] += dp[l][k][i][j];
                        }
                    }
                }
            }
        }
    }
    dp[0][0][r.len()][s.len()] + dp[1][0][r.len()][s.len()]
}

fn main() {
    input! {t: usize}

    println!(
        "{}",
        (0..t)
            .map(|_| {
                input! {mut s: marker::Bytes, l: usize, r: usize}
                s.iter_mut().for_each(|s| *s -= b'0');
                solve(&s, r) - solve(&s, l - 1)
            })
            .join("\n")
    )
}
