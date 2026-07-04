use proconio::*;
use rustc_hash::FxHashMap;

const M: i32 = 998244353;

fn main() {
    input! {n: usize, k: i64, a: [i64; n]}

    let mut cum = vec![0; n + 1];
    for (i, a) in a.into_iter().enumerate() {
        cum[i + 1] = cum[i] + a;
    }

    let mut memo = FxHashMap::default();
    memo.insert(0, 1);
    let mut sum = 1i32;
    let mut res = 0i32;
    for i in 1..=n {
        let tar = cum[i] - k;
        let t = (sum - memo.get(&tar).unwrap_or(&0)).rem_euclid(M);
        sum += t;
        sum %= M;
        res = t;
        let entry = memo.entry(cum[i]).or_insert(0);
        *entry += t;
        *entry %= M;
    }

    println!("{res}")
}
