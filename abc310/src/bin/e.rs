use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}
    let s = s
        .into_iter()
        .map(|c| (c - b'0') as usize)
        .collect::<Vec<_>>();

    let mut dp = vec![vec![0; 2]; n];
    dp[0][s[0]] = 1;

    for i in 1..n {
        for prev in 0..2 {
            let next = 1 - (s[i] & prev);
            dp[i][next] += dp[i - 1][prev];
        }

        dp[i][s[i]] += 1;
    }

    println!("{}", dp.into_iter().map(|v| v[1]).sum::<usize>())
}
