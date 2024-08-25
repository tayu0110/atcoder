use proconio::*;

const M: u32 = 1000000007;

fn main() {
    input! {x: marker::Bytes, s: marker::Bytes, t: marker::Bytes}

    let mut dp = vec![0; x.len() + 1];
    dp[0] = 1;

    for i in 0..x.len() {
        if i + s.len() <= x.len() && x[i..i + s.len()] == s[..] {
            dp[i + s.len()] += dp[i];
            dp[i + s.len()] %= M;
        }

        if i + t.len() <= x.len() && x[i..i + t.len()] == t[..] {
            dp[i + t.len()] += dp[i];
            dp[i + t.len()] %= M;
        }
    }

    println!("{}", dp[x.len()]);
}
