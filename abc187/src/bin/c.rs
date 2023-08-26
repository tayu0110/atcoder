use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, s: [String; n]}

    let mut ex = HashSet::new();
    let mut nex = HashSet::new();
    for s in s {
        if s.starts_with("!") {
            ex.insert(s[1..].to_string());
        } else {
            nex.insert(s);
        }
    }

    let un = ex.intersection(&nex).cloned().collect::<Vec<_>>();

    if un.len() > 0 {
        println!("{}", un[0])
    } else {
        println!("satisfiable")
    }
}
