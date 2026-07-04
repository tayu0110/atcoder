use proconio::*;

fn main() {
    input! {n: usize, a: [i64; n]}

    let mut dp = [0, i64::MIN];
    let mut new = [i64::MIN, i64::MIN];
    for a in a {
        new[0] = dp[1].max(dp[0] + a);
        new[1] = dp[0];
        dp = new;
    }

    println!("{}", dp[0])
}
