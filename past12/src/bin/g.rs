use proconio::*;
use std::collections::HashMap;

fn main() {
    input! { n: usize, l: usize, k: u32, s: [marker::Chars; n] }

    let mut res = 0;
    for i in 0u32..1 << l {
        if i.count_ones() != k {
            continue;
        }

        let mut map = HashMap::new();
        for s in &s {
            let mut ns = String::new();
            for j in 0..l {
                if i & (1 << j) == 0 {
                    ns.push(s[j]);
                } else {
                    ns.push('?');
                }
            }
            *map.entry(ns).or_insert(0) += 1;
        }

        res = res.max(*map.values().max().unwrap());
    }

    println!("{}", res)
}
