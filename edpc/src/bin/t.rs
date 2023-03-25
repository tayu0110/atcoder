use proconio::{input, marker::Chars};

const MOD: usize = 1000_000_007;

fn main() {
    input! {n: usize, s: Chars}

    let mut dp = vec![1usize; n + 1];
    for (i, c) in s.into_iter().enumerate() {
        let mut new = vec![0; n + 1];
        for j in 0..n {
            if c == '>' {
                new[0] += dp[j];
                new[0] %= MOD;
                new[j] += MOD - dp[j];
                new[j] %= MOD;
            } else {
                new[j] += dp[j];
                new[j] %= MOD;
                new[n - i - 1] += MOD - dp[j];
                new[n - i - 1] %= MOD;
            }
        }

        for j in 0..n {
            new[j + 1] += new[j];
            new[j + 1] %= MOD;
        }

        dp = new;
    }

    println!("{}", dp[0]);
}
