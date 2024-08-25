#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
    source::line::LineSource,
    *,
};

#[fastout]
fn main() {
    input! {n: usize}

    let mut map = std::collections::HashMap::new();
    for i in 2..=n {
        let mut now = i;
        for j in 2..=i {
            while now % j == 0 {
                *map.entry(j).or_insert(0) += 1;
                now /= j;
            }
        }

        if now != 1 {
            *map.entry(now).or_insert(0) += 1;
        }
    }

    let mut res = 0;
    let mut set = std::collections::HashSet::new();
    let keys = map.keys().cloned().collect::<Vec<_>>();
    for i in 0..keys.len() {
        let ki = *map.get(&keys[i]).unwrap();
        if ki >= 74 {
            res += 1;
        }
        for j in 0..keys.len() {
            if i == j {
                continue;
            }
            let kj = *map.get(&keys[j]).unwrap();
            if ki >= 2 && kj >= 24 {
                res += 1;
            }
            if ki >= 4 && kj >= 14 {
                res += 1;
            }
            for (k, key) in keys.iter().enumerate() {
                if i == k || j == k {
                    continue;
                }
                let kk = *map.get(key).unwrap();
                if i < j && ki >= 4 && kj >= 4 && kk >= 2 && !set.contains(&(i, j, k)) {
                    res += 1;
                    set.insert((i, j, k));
                }
                if i < k && ki >= 4 && kk >= 4 && kj >= 2 && !set.contains(&(i, k, j)) {
                    res += 1;
                    set.insert((i, k, j));
                }
                if j < k && kj >= 4 && kk >= 4 && ki >= 2 && !set.contains(&(j, k, i)) {
                    res += 1;
                    set.insert((j, k, i));
                }
            }
        }
    }

    println!("{}", res);
}
