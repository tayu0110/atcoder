use math::MathInt;
use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {n: usize, x: usize, a: [usize; n]}

    let mut cum = 0;
    let mut cnt = 0;
    let mut dp = vec![0usize; 1 << n];
    dp[0] = 1;
    for i in 0..1 << n {
        let mut sum = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                sum += a[j];
                sum %= M;
            }
        }

        if sum >= x {
            cum += sum * dp[i];
            cum %= M;
            cnt += dp[i];
            cnt %= M;
            continue;
        }

        for j in 0..n {
            if i & (1 << j) == 0 {
                dp[i | (1 << j)] += dp[i];
                dp[i | (1 << j)] %= M;
            }
        }
    }

    eprintln!("dp: {dp:?}, cum: {cum}, cnt: {cnt}");
    println!("{}", cum * cnt.inverse_mod(M).unwrap() % M)
}
