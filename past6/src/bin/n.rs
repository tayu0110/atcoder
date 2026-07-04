use proconio::*;

fn main() {
    input! {n: usize, h: usize, mut p: [(usize, usize); n]}

    p.sort_unstable_by(|&(a, b), &(c, d)| (c * b).cmp(&(a * d)));

    let mut dp = vec![0; h + 1];
    for (a, b) in p {
        dp[0] = dp[0].max(
            dp[..=b.min(h)]
                .iter()
                .enumerate()
                .map(|(i, d)| d + a * i)
                .max()
                .unwrap(),
        );
        for (i, next) in (b + 1..=h).zip(1..) {
            dp[next] = dp[next].max(dp[i] + a * i);
        }
    }

    println!("{}", dp.iter().max().unwrap())
}
