use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize, k: i64, a: [i64; n]}

    let mut sum = 0i64;
    let mut map = HashMap::new();
    for &a in &a {
        sum += a;
        *map.entry(sum).or_insert(0usize) += 1;
    }

    let mut res = 0;
    let mut sum = 0i64;
    for a in a {
        res += *map.get(&(k + sum)).unwrap_or(&0);
        sum += a;
        *map.entry(sum).or_insert(0) -= 1;
    }

    println!("{}", res)
}
