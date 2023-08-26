use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {s: marker::Chars}
    let len = s.len();

    if len % 2 == 1 {
        println!("0");
        return;
    }

    let mut dp = vec![vec![Modint::zero(); len + 1]; len + 1];
    dp[0][0] = Modint::one();

    for (i, &c) in s.iter().enumerate() {
        for open in 0..=i {
            let close = i - open;
            if open < close {
                continue;
            }
            let now = dp[open][close];

            if c == '(' {
                dp[open + 1][close] += now;
            } else if c == ')' {
                if open >= close + 1 {
                    dp[open][close + 1] += now;
                }
            } else {
                dp[open + 1][close] += now;
                if open >= close + 1 {
                    dp[open][close + 1] += now;
                }
            }
        }
    }

    println!("{}", dp[len / 2][len / 2]);
}
