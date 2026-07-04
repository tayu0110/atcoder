use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut map = FxHashMap::default();
    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut ret = 0usize;
    for &v in map.values() {
        ret += v * (v - 1) / 2 * (n - v);
    }
    println!("{}", ret)
}
