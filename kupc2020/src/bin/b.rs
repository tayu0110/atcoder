use proconio::*;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {n: usize, k: usize, v: [[u32; k]; n]}

    let mut dp = vec![1i64; k + 1];
    for i in 0..n - 1 {
        let mut new = vec![0i64; k + 1];
        let u = &v[i + 1];
        for (j, &w) in v[i].iter().enumerate() {
            let (mut l, mut r) = (-1, k as i32);
            while r - l > 1 {
                let m = (r + l) / 2;
                if u[m as usize] >= w {
                    r = m;
                } else {
                    l = m;
                }
            }

            new[r as usize] += dp[j];
            new[r as usize] = new[r as usize].rem_euclid(MOD);
            new[k] -= dp[j];
            new[k] = new[k].rem_euclid(MOD);
        }

        for j in 0..k {
            new[j + 1] += new[j];
            new[j + 1] = new[j + 1].rem_euclid(MOD);
        }

        dp = new;
    }

    println!("{}", dp.iter().fold(0, |s, v| (s + v) % MOD))
}
