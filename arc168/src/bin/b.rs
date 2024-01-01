use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    if a.iter().fold(0, |s, &a| s ^ a) != 0 {
        println!("-1");
        return;
    }

    let mut map = BTreeMap::new();
    for &a in &a {
        *map.entry(a).or_insert(0) += 1;
    }

    if map.values().all(|a| a % 2 == 0) {
        println!("0");
        return;
    }

    for (k, v) in map.into_iter().rev() {
        if v % 2 == 1 {
            println!("{}", k - 1);
            return;
        }
    }

    println!("0")
}
