use std::collections::BTreeMap;

use fenwick_tree::{Addition, FenwickTree};
use proconio::*;

const MAX: usize = 300001;

fn main() {
    input! {n: usize, q: usize}

    let mut ft = FenwickTree::<Addition<usize>>::new(MAX);
    ft.add(0, n);
    let mut map = BTreeMap::new();
    map.insert(0, n);

    let mut boxes = vec![0; n];

    for _ in 0..q {
        input! {ty: u8, x: usize}

        if ty == 1 {
            let old = boxes[x - 1];
            *map.entry(old).or_insert(0) -= 1;
            *map.entry(old + 1).or_insert(0) += 1;
            if *map.get(&old).unwrap() == 0 {
                map.remove(&old);
            }

            ft.add(old, usize::MAX);
            ft.add(old + 1, 1);
            boxes[x - 1] += 1;
        } else {
            let &min = map.first_key_value().unwrap().0;
            if min + x > MAX {
                println!("0");
            } else {
                println!("{}", ft.fold(min + x..));
            }
        }
    }
}
