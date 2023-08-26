use proconio::*;
use std::collections::HashSet;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut set = HashSet::new();
    for (u, v) in p {
        if u > n || v > n || v == u {
            println!("No");
            return;
        }

        if set.contains(&(u.min(v), u.max(v))) {
            println!("No");
            return;
        }
        set.insert((u.min(v), u.max(v)));
    }

    println!("Yes")
}
