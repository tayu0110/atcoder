use std::collections::{BTreeMap, BTreeSet};

use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut boxes = vec![BTreeMap::new(); n + 1];
    let mut ball = vec![BTreeSet::new(); 2000_10];
    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {i: usize, j: usize}

            *boxes[j].entry(i).or_insert(0) += 1;
            ball[i].insert(j);
        } else if t == 2 {
            input! {i: usize}
            for (k, v) in &boxes[i] {
                for _ in 0..*v {
                    println!("{}", k);
                }
            }
        } else {
            input! {i: usize}

            for &k in &ball[i] {
                println!("{}", k)
            }
        }
    }
}
