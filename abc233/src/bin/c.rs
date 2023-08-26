use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize, x: usize}

    let mut map = HashMap::new();
    map.insert(1, 1usize);
    for _ in 0..n {
        input! {l: usize, a: [usize; l]}

        let mut new = HashMap::new();
        for (k, v) in map {
            for &a in &a {
                if k <= x / a {
                    *new.entry(a * k).or_insert(0) += v;
                }
            }
        }

        map = new;
    }

    println!("{}", map.get(&x).unwrap_or(&0))
}
