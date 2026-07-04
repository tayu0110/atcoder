use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut a = vec![HashSet::new(); m];
    let mut t = vec![vec![]; n + 1];
    for i in 0..m {
        input! {k: usize, b: [usize; k]}
        for b in b {
            t[b].push(i);
            a[i].insert(b);
        }
    }

    input! {b: [usize; n]}
    let mut res = 0;
    for b in b {
        for &t in &t[b] {
            a[t].remove(&b);
            if a[t].is_empty() {
                res += 1;
            }
        }

        println!("{res}");
    }
}
