use proconio::*;
use std::sync::Mutex;

const MOD: u32 = 998244353;
static DP: Mutex<[u32; 5001]> = Mutex::new([0; 5001]);

#[fastout]
fn main() {
    input! {q: usize, k: usize, query: [(char, usize); q]}

    let dp = &mut DP.lock().unwrap()[..];
    dp[0] = 1;
    for (ty, x) in query {
        if x > k {
            println!("{}", dp[k]);
            continue;
        }

        if ty == '+' {
            for i in (0..k - x + 1).rev() {
                dp[i + x] += dp[i];
                dp[i + x] %= MOD;
            }
        } else {
            for i in 0..k - x + 1 {
                dp[i + x] += MOD - dp[i];
                dp[i + x] %= MOD;
            }
        }

        println!("{}", dp[k])
    }

    // eprintln!("dp: {dp:?}");
}
