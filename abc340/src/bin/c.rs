use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {n: usize}

    let mut t = BTreeMap::new();
    t.insert(n, 1);
    let mut res = 0;
    while let Some((k, v)) = t.pop_last() {
        res += k * v;
        if k / 2 >= 2 {
            *t.entry(k / 2).or_insert(0) += v;
        }
        if (k + 1) / 2 >= 2 {
            *t.entry((k + 1) / 2).or_insert(0) += v;
        }
    }

    println!("{res}")
}
