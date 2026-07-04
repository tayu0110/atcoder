use itertools::Itertools;
use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; m]}

    let mut map = FxHashMap::default();
    for b in b {
        *map.entry(b).or_insert(0) += 1;
    }
    let mut res = vec![];
    for a in a {
        let entry = map.entry(a).or_default();
        if *entry > 0 {
            *entry -= 1;
        } else {
            res.push(a);
        }
    }

    println!("{}", res.iter().join(" "))
}
