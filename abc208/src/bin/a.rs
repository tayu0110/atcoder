use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {a: usize, b: usize}
    let mut set = HashSet::new();
    set.insert(0);
    for _ in 0..a {
        let mut new = HashSet::new();
        for s in set {
            for i in 1..=6 {
                if s + i > b {
                    continue;
                }

                new.insert(s + i);
            }
        }

        set = new;
    }

    if set.contains(&b) {
        println!("Yes")
    } else {
        println!("No")
    }
}
