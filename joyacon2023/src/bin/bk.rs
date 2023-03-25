use proconio::input;

fn main() {
    const MOD: usize = 998244353;
    input! {n: usize, m: usize, k: usize}

    let mut dp = vec![vec![0; k + 1]; m + 1];
    dp[0][0] = 1;
    for _ in 0..n {
        let mut new = vec![vec![0; k + 1]; m + 1];
        for i in 1..=m {
            for j in 0..=m {
                for l in 0..=k {
                    if dp[j][l] == 0 {
                        continue;
                    }
                    if l + i > k {
                        continue;
                    }

                    new[i][l + i] += dp[j][l];
                    new[i][l + i] %= MOD;
                }
            }
        }
        dp = new;
    }

    let res = dp.iter().flatten().fold(0, |s, v| (s + v) % MOD);
    println!("{}", res);
}
