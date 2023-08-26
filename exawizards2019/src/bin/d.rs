use proconio::input;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize, x: usize, mut s: [usize; n]}
    s.sort();

    let mut dp = vec![Modint::zero(); x + 1];
    for i in 0..=x {
        dp[i] = Modint::raw(i as u32);
    }

    for i in 0..n {
        let mut next = vec![Modint::zero(); x + 1];
        for x in 0..=x {
            next[x] += dp[x % s[i]];
            next[x] += dp[x] * Modint::raw(i as u32);
        }
        dp = next;
    }

    println!("{}", dp[x]);
}
