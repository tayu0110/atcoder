use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::*;

fn swap(x: usize, a: &mut [usize], set: &mut BTreeSet<usize>) {
    a.swap(x, x + 1);

    for i in x - 1..(x + 2).min(a.len() - 1) {
        if a[i] < a[i + 1] {
            set.remove(&i);
        } else {
            set.insert(i);
        }
    }
}

fn main() {
    input! {n: usize, q: usize, query: [(usize, usize, usize); q]}

    let mut set = BTreeSet::new();
    let mut a = (0..=n).collect::<Vec<_>>();
    for (t, x, y) in query {
        if t == 1 {
            swap(x, &mut a, &mut set);
        } else {
            while let Some(&i) = set.range(x..y).next() {
                swap(i, &mut a, &mut set);
            }
        }
    }

    println!("{}", a.iter().skip(1).join(" "))
}
