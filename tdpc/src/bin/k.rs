use proconio::*;

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let mut p = p
        .into_iter()
        .map(|(x, r)| (x - r, x + r))
        .collect::<Vec<_>>();
    p.sort();

    let mut dp = vec![std::i64::MAX; n];
    for (_, k) in p {
        let (mut l, mut r) = (-1, n as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if -k <= dp[m as usize] {
                r = m;
            } else {
                l = m;
            }
        }
        dp[r as usize] = -k;
    }

    println!(
        "{}",
        dp.into_iter().take_while(|&d| d < std::i64::MAX).count()
    );
}
