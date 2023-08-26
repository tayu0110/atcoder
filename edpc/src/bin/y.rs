use proconio::*;
use static_modint::{combination, Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {h: usize, w: usize, n: usize, mut p: [(usize, usize); n]}

    p.push((h, w));
    p.sort_by_key(|&(l, r)| l + r);

    let n = p.len();

    let mut dp = vec![Modint::zero(); n];
    let com = combination(h as u32 + w as u32 + 10);
    for (i, &(r, c)) in p.iter().enumerate() {
        dp[i] = com(r - 1 + c - 1, r - 1);
        for (j, &(y, x)) in p.iter().take(i).enumerate() {
            if y > r || x > c {
                continue;
            }

            let k = dp[j] * com(r - y + c - x, r - y);
            dp[i] -= k;
        }
    }

    println!("{}", dp[n - 1])
}
