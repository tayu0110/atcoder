use proconio::{input, marker::Chars};

const MOD: usize = 998244353;

fn main() {
    input! {n: usize, x: Chars}

    let mut dp = vec![0; n + 1];
    let mut cum = vec![0; n + 1];
    cum[0] = 1;
    for i in 0..n {
        let c = (x[i] as u8 - b'0') as usize;

        dp[i + 1] += dp[i] * 10 + cum[i] * c;
        dp[i + 1] %= MOD;

        cum[i + 1] = cum[i] + dp[i + 1];
        cum[i + 1] %= MOD;
    }

    println!("{}", dp[n]);
}
