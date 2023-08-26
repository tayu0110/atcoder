use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

const MAX: usize = 1 << 8;
const MASK: usize = MAX - 1;

fn main() {
    input! {n: usize, l: usize, w: [marker::Bytes; n]}

    let mut dp = vec![vec![Modint::zero(); MAX]; l + 1];
    dp[0][0] = Modint::one();

    for i in 0..l {
        for s in &w {
            if i + s.len() > l {
                continue;
            }
            for j in 0..MAX {
                if dp[i][j] == Modint::zero() {
                    continue;
                }
                let now = dp[i][j];
                let mut next = j;
                for &c in s {
                    next = (next << 1) | (&c - b'0') as usize;
                }
                eprintln!("j: {j:0b}, next: {next:0b}");
                next &= MASK;
                dp[i + s.len()][next] += now;
            }
        }
    }

    println!("{}", dp[l].iter().fold(Modint::zero(), |s, v| s + *v))
}
