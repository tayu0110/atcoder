use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize, mut s: [marker::Bytes; n]}

    s.sort_unstable_by_key(|s| s.len());

    let mut res = n;
    {
        let mut map = HashMap::new();
        for s in &s {
            *map.entry(s).or_insert(0) += 1;
        }
        for v in map.values() {
            res += v * (v - 1) / 2;
        }
    }

    let mut map = HashMap::new();
    for s in s {
        let mut u = vec![];
        for i in 0..1 << s.len() {
            let mut t = vec![];
            for j in 0..s.len() {
                if i & (1 << j) != 0 {
                    t.push(s[j]);
                }
            }
            u.push(t);
        }

        u.sort_unstable();
        u.dedup();

        for t in u {
            res += map.get(&t).unwrap_or(&0);
        }

        *map.entry(s).or_insert(0) += 1;
    }

    println!("{res}")
}
