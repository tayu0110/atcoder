use proconio::*;

const MAX: usize = 100010;

fn main() {
    input! {n: usize, t: [usize; n]}

    let mut dp = vec![false; MAX];
    dp[0] = true;
    let mut sum = 0;
    for t in t {
        sum += t;
        for i in (0..MAX).rev() {
            if dp[i] && i + t < MAX {
                dp[i + t] = true;
            }
        }
    }

    println!(
        "{}",
        dp.into_iter()
            .enumerate()
            .filter(|(_, f)| *f)
            .map(|(i, _)| i.max(sum - i))
            .min()
            .unwrap()
    )
}
