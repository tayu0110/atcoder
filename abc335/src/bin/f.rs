use proconio::*;
use rustc_hash::FxHashMap;

type HashMap = FxHashMap<u32, u32>;

const M: u32 = 998244353;

fn main() {
    input! {n: usize, a: [u32; n]}

    let mut map = vec![HashMap::default(); n];
    let mut dp = vec![0; n];
    dp[0] = 1;

    for (i, a) in a.into_iter().enumerate() {
        {
            let entry = map[i].entry(a).or_insert(0);
            *entry += dp[i];
            *entry %= M;
        }

        let (f, b) = map.split_at_mut(i + 1);
        for (&k, &v) in &f[i] {
            if k as usize - 1 < b.len() {
                dp[i + k as usize] += v;
                dp[i + k as usize] %= M;
                let entry = b[k as usize - 1].entry(k).or_insert(0);
                *entry += v;
                *entry %= M;
            }
        }
        f[i].clear();
    }

    println!("{}", dp.iter().fold(0, |s, v| (s + v) % M))
}
