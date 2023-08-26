use proconio::*;

const MOD: usize = 998244353;

fn main() {
    input! {s: marker::Chars, k: usize}

    let mut t = vec![];
    {
        let mut cnt = 1;
        for c in s {
            if c == '0' {
                t.push(cnt);
                cnt = 0;
            } else {
                cnt += 1;
            }
        }
        if cnt > 0 {
            t.push(cnt);
        }
    }
    t.reverse();

    let k = k.min(t.iter().sum());
    let mut dp = vec![vec![0; k + 1]; k + 1];
    dp[0][0] = 1;
    for i in 1..=t.len() {
        let mut new = vec![vec![0; k + 1]; k + 1];
        for s in 0..=k {
            let mut cur = 0;
            for r in 0..=s {
                cur = (cur + dp[s][r]) % MOD;
                new[s][r] = cur;
            }
        }
        for r in 0..=k {
            let mut cur = 0;
            for s in r..=k {
                if s - r > t[i - 1] {
                    cur = (cur + MOD - dp[s - t[i - 1] - 1][r]) % MOD;
                }
                new[s][r] = (new[s][r] + cur) % MOD;
                cur = (cur + dp[s][r]) % MOD;
            }
        }

        dp = new;
    }

    println!(
        "{}",
        (0..=k).map(|i| dp[i][i]).fold(0, |s, v| (s + v) % MOD)
    )
}
