use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn ord(c: u8) -> usize {
    (c - b'0') as usize
}

fn main() {
    input! {d: usize, n: marker::Bytes}
    let len = n.len();

    let mut dp = vec![vec![vec![Modint::zero(); 2]; d]; len];
    for i in 0..ord(n[0]) {
        dp[0][i][0] = Modint::one();
    }
    dp[0][ord(n[0])][1] = Modint::one();

    for i in 1..len {
        let c = ord(n[i]);
        for j in 0..d {
            for k in 0..10 {
                let next = (j + k) % d;
                if k == c {
                    let new = dp[i - 1][j][1];
                    dp[i][next][1] += new;

                    let new = dp[i - 1][j][0];
                    dp[i][next][0] += new;
                } else if k < c {
                    let new = dp[i - 1][j][0] + dp[i - 1][j][1];
                    dp[i][next][0] += new;
                } else {
                    let new = dp[i - 1][j][0];
                    dp[i][next][0] += new;
                }
            }
        }
    }
    // eprintln!("{dp:?}");

    println!("{}", dp[len - 1][0][0] + dp[len - 1][0][1] - Modint::one())
}
