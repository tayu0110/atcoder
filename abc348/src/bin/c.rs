use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut map = FxHashMap::default();
    for (a, c) in p {
        let e = map.entry(c).or_insert(a);
        *e = a.min(*e);
    }

    println!("{}", map.values().max().unwrap());
}
