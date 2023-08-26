use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, p: [(String, usize); n]}

    let mut max = 0;
    let mut res = 0;
    let mut set = HashSet::new();
    for (i, (s, t)) in p.into_iter().enumerate() {
        if !set.contains(&s) && t > max {
            res = i + 1;
            max = t;
        }

        set.insert(s);
    }

    println!("{}", res)
}
