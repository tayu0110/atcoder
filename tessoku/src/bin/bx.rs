use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize, w: u32, l: u32, r: u32, mut x: [u32; n]}
    x.insert(0, 0);
    x.push(w);
    let n = x.len();

    let mut dp = vec![Modint::zero(); n + 1].into_boxed_slice();
    dp[0] = Modint::one();
    dp[1] -= Modint::one();

    for (i, &now) in x.iter().enumerate() {
        let l = x.partition_point(|&x| x < now + l);
        let r = x.partition_point(|&x| x <= now + r);
        dp[l] += dp[i];
        dp[r] -= dp[i];

        dp[i + 1] += dp[i];
    }

    println!("{}", dp[n - 1])
}
