use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {p: [(String, String)]}
    let mut set = HashSet::new();
    for (s, t) in p {
        if set.contains(&(s.clone(), t.clone())) {
            println!("Yes");
            return;
        }

        set.insert((s, t));
    }

    println!("No")
}
