use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, c: [String; n], d: [String; m], p: [usize; m+1]}

    let mut map = HashMap::new();
    for (i, d) in d.into_iter().enumerate() {
        map.insert(d, p[i + 1]);
    }

    println!(
        "{}",
        c.into_iter()
            .map(|c| {
                if map.contains_key(&c) {
                    *map.get(&c).unwrap()
                } else {
                    p[0]
                }
            })
            .sum::<usize>()
    )
}
