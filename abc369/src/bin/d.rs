use proconio::*;

fn main() {
    input! {n: usize, a: [i64; n]}

    let mut dp = [0, -1];
    for a in a {
        let mut new = [-1; 2];
        for i in 0..2 {
            if dp[i] >= 0 {
                new[i] = new[i].max(dp[i]);
                let next = (i + 1) % 2;
                let t = dp[i] + a + if next == 0 { a } else { 0 };
                new[next] = new[next].max(t);
            }
        }
        dp = new;
    }

    println!("{}", dp[0].max(dp[1]))
}
