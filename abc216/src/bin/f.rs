use proconio::*;

const MAX: usize = 5010;
const MOD: usize = 998244353;

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n]}

    let mut p = a.into_iter().zip(b).collect::<Vec<_>>();
    p.sort();

    let mut dp = vec![0; MAX];
    dp[0] = 1;
    let mut res = 0;
    for (a, b) in p {
        for i in (0..MAX).rev() {
            if i + b < MAX {
                dp[i + b] += dp[i];
                dp[i + b] %= MOD;

                if i + b <= a {
                    res += dp[i];
                    res %= MOD;
                }
            }
        }
    }

    println!("{}", res)
}
