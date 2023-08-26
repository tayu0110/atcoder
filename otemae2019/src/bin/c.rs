use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n]}

    let mut map = HashMap::new();
    for b in b {
        *map.entry(b).or_insert(0) += 1;
    }

    let mut ma = HashMap::new();
    let mut res = std::usize::MAX;
    for a in a {
        *ma.entry(a).or_insert(0) += 1;
        let t = *ma.get(&a).unwrap();

        let r = *map.get(&a).unwrap_or(&0);
        res = res.min(r / t);

        println!("{}", res)
    }
}
