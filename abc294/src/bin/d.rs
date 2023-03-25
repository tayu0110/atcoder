use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {_n: usize, q: usize}

    let mut next = 1;
    let mut set = BTreeSet::new();
    let mut res = vec![];

    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            set.insert(next);
            next += 1;
        } else if t == 2 {
            input! {x: usize}
            set.remove(&x);
        } else {
            if let Some(&r) = set.iter().next() {
                res.push(r);
            }
        }
    }

    println!("{}", res.into_iter().join("\n"))
}
