use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
    }

    let mut reach = 0;
    let mut check = vec![false; n];
    let mut nt = BTreeSet::new();
    nt.insert(0);
    for start in 0..n {
        while let Some(&now) = nt.first().filter(|&&n| n <= start) {
            if check[now] {
                nt.remove(&now);
                continue;
            }

            check[now] = true;
            reach += 1;

            for &to in &t[now] {
                if !check[to] {
                    nt.insert(to);
                }
            }
        }

        if reach != start + 1 {
            println!("-1")
        } else {
            println!("{}", nt.len())
        }
    }
}
