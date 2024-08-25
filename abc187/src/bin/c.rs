use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, s: [String; n]}

    let mut ex = HashSet::new();
    let mut nex = HashSet::new();
    for s in s {
        if let Some(rem) = s.strip_prefix('!') {
            ex.insert(rem.to_string());
        } else {
            nex.insert(s);
        }
    }

    let un = ex.intersection(&nex).cloned().collect::<Vec<_>>();

    if !un.is_empty() {
        println!("{}", un[0])
    } else {
        println!("satisfiable")
    }
}
