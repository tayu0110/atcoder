use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {n: usize, q: usize, a: [usize; n]}

    let mut map = BTreeMap::new();
    for &a in &a {
        *map.entry(a).or_insert(0) += 1;
    }

    for _ in 0..q {
        input! {k: usize, b: [usize; k]}

        for &b in &b {
            *map.entry(a[b - 1]).or_insert(0) -= 1;
        }

        println!("{}", map.iter().filter(|v| *v.1 > 0).next().unwrap().0);
        for b in b {
            *map.entry(a[b - 1]).or_insert(0) += 1;
        }
    }
}
