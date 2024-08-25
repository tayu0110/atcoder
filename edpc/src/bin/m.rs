use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut dp = vec![0i64; k + 1];
    dp[0] = 1;
    for a in a {
        let mut new = vec![0i64; k + 1];
        for i in 0..=k {
            new[i] += dp[i];
            new[i] %= MOD;
            if i + a < k {
                new[i + a + 1] -= dp[i];
                new[i + a + 1] = new[i + a + 1].rem_euclid(MOD);
            }
        }

        for i in 0..k {
            new[i + 1] += new[i];
            new[i + 1] %= MOD;
        }

        dp = new;
    }

    println!("{}", dp[k]);
}
