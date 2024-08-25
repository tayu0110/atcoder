use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n]}
    a.iter_mut().for_each(|a| *a -= 1);

    let mut cum = vec![vec![0; n + 1]; m];
    for (i, &a) in a.iter().enumerate() {
        for j in 0..m {
            cum[j][i + 1] = cum[j][i] + (j == a) as u32;
        }
    }

    let mut dp = vec![u32::MAX; 1 << m];
    dp[0] = 0;
    for i in 0..1 << m {
        let mut s = 0;
        for j in 0..m {
            if i & (1 << j) != 0 {
                s += cum[j][n] as usize;
            }
        }

        for j in 0..m {
            if i & (1 << j) == 0 {
                let next = i | (1 << j);
                let t = cum[j][n] as usize;
                dp[next] = dp[next].min(dp[i] + t as u32 - (cum[j][s + t] - cum[j][s]));
            }
        }
    }

    println!("{}", dp[(1 << m) - 1])
}
