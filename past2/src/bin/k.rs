use proconio::*;

fn main() {
    input! {n: usize, s: marker::Chars, c: [usize; n], d: [usize; n]}

    let mut dp = vec![usize::MAX; n + 1];
    dp[0] = 0;
    for ((s, c), d) in s.into_iter().zip(c).zip(d) {
        let mut new = vec![usize::MAX; n + 1];
        for i in 0..n + 1 {
            if dp[i] == usize::MAX {
                continue;
            }

            new[i] = new[i].min(dp[i] + d);
            new[i + 1] = new[i + 1].min(dp[i] + c * (s == ')') as usize);
            if i > 0 {
                new[i - 1] = new[i - 1].min(dp[i] + c * (s == '(') as usize);
            }
        }

        dp = new;
    }

    println!("{}", dp[0])
}
