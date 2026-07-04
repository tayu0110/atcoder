use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut memo = BTreeMap::new();
    for i in 0..n {
        *memo.entry(a[i]).or_insert(0) += 1;
    }

    for (k, v) in memo.into_iter().rev() {
        if v == 1 {
            println!("{}", a.iter().position(|&a| a == k).unwrap() + 1);
            return;
        }
    }

    println!("-1")
}
