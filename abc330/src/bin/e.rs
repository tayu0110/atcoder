use std::collections::{BTreeSet, HashMap};

use proconio::*;

fn main() {
    input! {n: usize, q: usize, mut a: [usize; n]}

    let mut set = (0..n + 1).collect::<BTreeSet<_>>();
    let mut map = HashMap::new();
    for &a in &a {
        set.remove(&a);
        *map.entry(a).or_insert(0) += 1;
    }

    for _ in 0..q {
        input! {i: usize, x: usize}

        let old = a[i - 1];
        a[i - 1] = x;

        *map.entry(old).or_insert(1) -= 1;
        if *map.get(&old).unwrap() == 0 {
            map.remove(&old);
            set.insert(old);
        }

        if !map.contains_key(&x) {
            set.remove(&x);
        }
        *map.entry(x).or_insert(0) += 1;

        println!("{}", set.first().unwrap())
    }
}
