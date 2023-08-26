use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize, s: [String; n]}

    let mut map = HashMap::new();
    for s in s {
        *map.entry(s).or_insert(0) += 1;
    }

    println!("AC x {}", map.get("AC").unwrap_or(&0));
    println!("WA x {}", map.get("WA").unwrap_or(&0));
    println!("TLE x {}", map.get("TLE").unwrap_or(&0));
    println!("RE x {}", map.get("RE").unwrap_or(&0))
}
