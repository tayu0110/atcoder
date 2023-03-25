use proconio::{input, marker::Chars};

const MOD: usize = 1000_000_007;

fn main() {
    input! {h: usize, w: usize, a: [Chars; h]}

    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                continue;
            }

            if j + 1 < w && a[i][j + 1] != '#' {
                dp[i][j + 1] += dp[i][j];
                dp[i][j + 1] %= MOD;
            }
            if i + 1 < h && a[i + 1][j] != '#' {
                dp[i + 1][j] += dp[i][j];
                dp[i + 1][j] %= MOD;
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
