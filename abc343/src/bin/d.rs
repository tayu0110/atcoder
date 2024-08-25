use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, t: usize, p: [(usize, usize); t]}

    let mut map = FxHashMap::default();
    map.insert(0, n);
    let mut point = vec![0; n];
    for (a, b) in p {
        let old = point[a - 1];
        point[a - 1] += b;

        *map.entry(old).or_insert(0) -= 1;
        if *map.get(&old).unwrap() == 0 {
            map.remove(&old);
        }

        *map.entry(point[a - 1]).or_insert(0) += 1;

        println!("{}", map.len());
    }
}
