use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut map = BTreeMap::new();
    for (s, c) in p {
        map.insert(s, c);
    }

    let mut res = 0;
    while let Some((s, c)) = map.pop_first() {
        res += c % 2;
        if c / 2 > 0 {
            *map.entry(s * 2).or_insert(0) += c / 2;
        }
    }

    println!("{}", res);
}
