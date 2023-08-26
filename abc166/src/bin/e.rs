use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {n: usize, a: [i64; n]}

    let mut res = 0;
    let mut map = HashMap::new();
    for (i, a) in a.into_iter().enumerate() {
        let k = i as i64 - a;
        res += *map.get(&k).unwrap_or(&0);
        *map.entry(a + i as i64).or_insert(0usize) += 1;
    }

    println!("{}", res)
}
