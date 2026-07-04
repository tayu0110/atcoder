use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut map = FxHashMap::default();
    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut t = map.into_iter().collect::<Vec<_>>();
    t.sort_unstable_by_key(|(k, v)| k * v);
    let n = t.len().saturating_sub(k);
    t.truncate(n);
    println!("{}", t.into_iter().map(|(k, v)| k * v).sum::<usize>())
}
