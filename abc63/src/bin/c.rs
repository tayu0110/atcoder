use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}
    const MAX: usize = 100 * 100;

    let mut dp = vec![false; MAX + 1];
    dp[0] = true;
    for s in a {
        for i in (0..MAX).rev() {
            if dp[i] && i + s <= MAX {
                dp[i + s] = true;
            }
        }
    }

    println!(
        "{}",
        dp.into_iter()
            .enumerate()
            .rev()
            .skip_while(|(i, f)| !f || i % 10 == 0)
            .map(|(i, _)| i)
            .next()
            .unwrap_or(0)
    );
}
