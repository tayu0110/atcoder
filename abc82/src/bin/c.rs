use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut map = HashMap::new();
    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut res = 0;
    for (k, v) in map {
        if k == 1 && v > 0 {
            res += v - k;
        } else {
            res += v % k;
        }
    }

    println!("{}", res)
}
