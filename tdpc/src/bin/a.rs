use proconio::*;

const MAX: usize = 10010;

fn main() {
    input! {n: usize, p: [usize; n]}

    let mut dp = vec![false; MAX];
    dp[0] = true;
    for p in p {
        for i in (0..MAX).rev() {
            if dp[i] {
                dp[i + p] = true;
            }
        }
    }

    println!("{}", dp.into_iter().filter(|f| *f).count())
}
