use itertools::Itertools;
use proconio::*;
use std::collections::HashSet;

fn main() {
    input! {n: usize, event: [(usize, usize); n]}

    let mut p = vec![vec![]; 200001];
    let mut set = HashSet::new();
    for (i, &(t, x)) in event.iter().enumerate() {
        if t == 1 {
            p[x].push(i);
        } else {
            if p[x].is_empty() {
                println!("-1");
                return;
            }

            let r = p[x].pop().unwrap();
            set.insert(r);
        }
    }

    let mut max = 0;
    let mut have = 0;
    let mut res = vec![];
    for (i, (t, _)) in event.into_iter().enumerate() {
        if t == 1 {
            if set.contains(&i) {
                res.push(1);
                have += 1;
            } else {
                res.push(0);
            }
        } else {
            have -= 1;
        }
        max = max.max(have);
    }
    println!("{max}");
    println!("{}", res.iter().join(" "))
}
