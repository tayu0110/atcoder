use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [[u16; n]; m]}

    let a = a
        .into_iter()
        .map(|v| v.into_iter().fold(0, |s, v| (s << 1) | v))
        .collect::<Vec<_>>();
    let mut dp = vec![u8::MAX; 1 << n];
    dp[0] = 0;
    for a in a {
        for i in (0..1 << n).rev() {
            if dp[i as usize] < u8::MAX {
                let next = i | a;
                dp[next as usize] = dp[next as usize].min(dp[i as usize] + 1);
            }
        }
    }

    println!("{}", dp[(1 << n) - 1] as i8);
}
