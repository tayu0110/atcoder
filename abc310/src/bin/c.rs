use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, s: [String; n]}

    let mut set = HashSet::new();
    for s in s {
        let r = s.chars().rev().collect::<String>();
        set.insert(s.min(r));
    }

    println!("{}", set.len())
}
