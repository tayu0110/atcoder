use proconio::*;

const MAX: usize = 200050;
const MOD: i64 = 998244353;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut cnt = vec![0; MAX];
    for &a in &a {
        cnt[a] += 1;
    }

    let mut dp = vec![1];
    for i in 0..MAX - 1 {
        let mut new = vec![0; cnt[i + 1] + dp.len() / 2 + 2];
        for j in 0..dp.len() {
            if dp[j] == 0 {
                continue;
            }

            new[cnt[i + 1]] += dp[j];
            new[cnt[i + 1]] %= MOD;
            new[cnt[i + 1] + j / 2 + 1] -= dp[j];
            new[cnt[i + 1] + j / 2 + 1] = new[cnt[i + 1] + j / 2 + 1].rem_euclid(MOD);
        }

        for i in 0..cnt[i + 1] + dp.len() / 2 + 1 {
            new[i + 1] += new[i];
            new[i + 1] %= MOD;
        }

        new.pop().unwrap();
        dp = new;
    }

    println!("{}", dp.into_iter().fold(0, |s, v| (s + v) % MOD))
}
