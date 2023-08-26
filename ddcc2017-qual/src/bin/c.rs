use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {n: usize, c: usize, mut l: [usize; n]}

    let mut map = BTreeMap::new();
    for &l in &l {
        *map.entry(l).or_insert(0) += 1;
    }

    let mut res = 0;
    l.sort();
    while let Some(l) = l.pop() {
        if !map.contains_key(&l) {
            continue;
        }
        *map.entry(l).or_insert(0) -= 1;
        if *map.entry(l).or_insert(0) == 0 {
            map.remove(&l);
        }

        let rem = c - l;
        let mut remove = None;
        if let Some((nl, nc)) = map.range_mut(..rem).next_back() {
            *nc -= 1;
            if *nc == 0 {
                remove = Some(*nl);
            }
        }
        if let Some(remove) = remove {
            map.remove(&remove);
        }
        res += 1;
    }

    println!("{}", res)
}
