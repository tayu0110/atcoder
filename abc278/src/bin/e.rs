#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {h: usize, w: usize, n: usize, ht: usize, wt: usize, a: [[usize; w]; h]}

    let mut t = vec![vec![]; n+1];

    let mut set = std::collections::HashSet::new();
    for i in 0..h {
        for j in 0..w {
            t[a[i][j]].push((i, j));
            set.insert(a[i][j]);
        }
    }

    let mut res = vec![vec![set.len(); w-wt+1]; h-ht+1];
    for i in 1..=n {
        if t[i].is_empty() {
            continue;
        }
        let (u, d, l, r) = t[i].iter().fold((h, 0, w, 0), |(u, d, l, r), &(s, t)| 
            (
                std::cmp::min(u, s),
                std::cmp::max(d, s),
                std::cmp::min(l, t),
                std::cmp::max(r, t)
            ));

        if d - u > ht || r - l > wt {
            continue;
        }
        
        let nu = d.saturating_sub(ht-1);
        let nl = r.saturating_sub(wt-1);

        for i in nu..=std::cmp::min(u, h-ht) {
            for j in nl..=std::cmp::min(l, w-wt) {
                res[i][j] -= 1;
            }
        }
    }

    for v in res {
        println!("{}", v.iter().join(" "));
    }
}
