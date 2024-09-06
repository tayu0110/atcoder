use proconio::*;

fn main() {
    input! {n: usize, k: usize, t: usize, p: [usize; n]}

    let mut cum = vec![0; n + 1];
    for (i, p) in p.into_iter().enumerate() {
        cum[i + 1] = cum[i] + p;
    }

    let mut dp = vec![0; n + 1];
    for s in (0..=n).step_by(t).take(k) {
        let mut new = vec![usize::MAX; n + 1];
        for i in (s..=n).take_while(|i| i + t <= n) {
            new[i + t] = new[i + t].min(dp[i] + cum[i + t] - cum[i]);
        }

        for i in s.max(1)..=n {
            new[i] = new[i].min(new[i - 1]);
        }

        dp = new;
    }

    println!("{}", dp[n])
}
