use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, a: [u32; n]}

    let mut map = FxHashMap::default();

    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    println!(
        "{}",
        map.values()
            .map(|&a| a as u64 * (a - 1) as u64 / 2)
            .sum::<u64>()
    )
}
