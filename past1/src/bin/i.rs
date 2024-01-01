use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(marker::Bytes, usize); m]}

    let mut dp = vec![usize::MAX; 1 << n];
    dp[0] = 0;
    for (s, c) in p.into_iter().map(|(s, c)| {
        (
            s.into_iter()
                .fold(0, |s, v| (s << 1) | (v == b'Y') as usize),
            c,
        )
    }) {
        for i in (0..(1 << n)).rev() {
            if dp[i] < usize::MAX {
                dp[i | s] = dp[i | s].min(dp[i] + c);
            }
        }
    }

    println!("{}", dp[(1 << n) - 1] as i64)
}
