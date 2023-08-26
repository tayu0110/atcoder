use std::collections::HashMap;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let mut map = HashMap::new();
    for c in s {
        *map.entry(c).or_insert(0) += 1;
    }

    if map.values().all_equal() {
        println!("Yes")
    } else {
        println!("No")
    }
}
