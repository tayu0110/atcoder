#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, s: Chars, t: Chars}
    const MOD: usize = 1000000007;

    let mut dp = vec![0; n];
    for (i, (cs, ct)) in s.iter().zip(t.iter()).enumerate() {
        if i == 0 {
            if cs == ct {
                dp[i] = 3;
            } else {
                dp[i] = 6;
            }
        } else {
            let (ps, pt) = (&s[i-1], &t[i-1]);

            if ps == cs && pt == ct {
                dp[i] = dp[i-1];
            } else if cs == ct {
                if ps == pt {
                    dp[i] = dp[i-1] * 2;
                } else {
                    dp[i] = dp[i-1];
                }
            } else {
                if ps == pt {
                    dp[i] = dp[i-1] * 2;
                } else {
                    dp[i] = dp[i-1] * 3;
                }
            }
        }

        dp[i] %= MOD;
    }

    println!("{}", dp[n-1]);
}
