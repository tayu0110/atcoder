use proconio::*;
use rustc_hash::FxHashMap;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;
type HashMap<K, V> = FxHashMap<K, V>;

fn main() {
    input! {n: usize, m: usize, s: [u8; n]}

    let mut dp = HashMap::default();
    dp.insert((0, 0), Modint::one());
    for s in s {
        let mut new = HashMap::default();
        if s == 0 {
            for ((i, j), v) in dp {
                for k in 1..=j + 1 {
                    let ni = i + k;
                    let nj = j - (k - 1);
                    *new.entry((ni, nj)).or_insert(Modint::zero()) += v;
                }
            }
        } else {
            for ((i, j), v) in dp {
                *new.entry((i, j)).or_insert(Modint::zero()) +=
                    v * Modint::raw(j as u32) + v * Modint::raw(m.saturating_sub(n - 1) as u32);
                if i + j + 1 < n {
                    *new.entry((i, j + 1)).or_insert(Modint::zero()) += v;
                }
            }
        }
        dp = new;
    }
}
