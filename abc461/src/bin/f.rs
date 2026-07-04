use math::MathInt;
use proconio::*;

const M: usize = 998244353;
const MAX: usize = 15;

fn main() {
    input! {n: usize}

    let mut divs = n.divisors();
    divs.sort_unstable();
    // eprintln!("divs: {divs:?}");

    let mut count = vec![vec![0; divs.len()]; MAX + 1];
    let mut dp = vec![vec![0; divs.len()]; MAX + 1];
    count[0][0] = 1;
    for &d in &divs {
        for i in (0..MAX).rev() {
            for j in (0..divs.len()).rev() {
                let k = d.saturating_mul(divs[j]);
                if let Ok(k) = divs.binary_search(&k) {
                    count[i + 1][k] += count[i][j] * (i + 1);
                    count[i + 1][k] %= M;
                    dp[i + 1][k] += count[i][j] * (i + 1) % M * (d % M);
                    dp[i + 1][k] += dp[i][j] * (i + 1);
                    dp[i + 1][k] %= M;
                }
            }
        }
    }
    // eprintln!("count: {count:?}");
    // eprintln!("dp: {dp:?}");

    let mut ret = 0;
    for i in 0..=MAX {
        ret += dp[i][divs.len() - 1];
        ret %= M;
    }
    println!("{ret}")
}
