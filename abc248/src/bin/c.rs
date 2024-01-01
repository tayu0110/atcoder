use proconio::*;

const MOD: usize = 998244353;

fn main() {
    input! {n: usize, m: usize, k: usize}

    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for _ in 0..n {
        let mut new = vec![0; k + 1];
        for i in (0..k).rev() {
            for j in 1..=m {
                if i + j <= k {
                    new[i + j] += dp[i];
                    new[i + j] %= MOD;
                }
            }
        }
        dp = new;
    }

    println!("{}", dp.iter().sum::<usize>() % MOD)
}
