use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, parts: [(i32, usize, usize); n]}

    let mut dp = FxHashMap::default();
    dp.insert(0i32, 0);
    for (w, h, b) in parts {
        let mut next = FxHashMap::default();
        for (k, v) in dp {
            let entry = next.entry(k - w).or_default();
            *entry = (v + h).max(*entry);

            let entry = next.entry(k + w).or_default();
            *entry = (v + b).max(*entry);
        }

        dp = next;
    }

    println!(
        "{}",
        dp.into_iter()
            .filter_map(|(k, v)| (k >= 0).then_some(v))
            .max()
            .unwrap()
    )
}
