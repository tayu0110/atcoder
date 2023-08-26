use std::collections::HashMap;

use proconio::*;

fn bin_search(p: usize, a: &[usize]) -> usize {
    let n = a.len();
    let (mut l, mut r) = (0, n);
    while r - l > 1 {
        let m = (r + l) / 2;
        if a[m] < p {
            l = m;
        } else {
            r = m;
        }
    }

    l
}

fn main() {
    input! {w: usize, h: usize, n: usize, p: [(usize, usize); n], a: usize, mut a: [usize; a], b: usize, mut b: [usize; b]}
    let (na, nb) = (a.len(), b.len());

    a.insert(0, 0);
    b.insert(0, 0);
    a.push(w);
    b.push(h);

    let mut map = HashMap::new();
    for (p, q) in p {
        let (nw, nh) = (bin_search(p, &a), bin_search(q, &b));

        *map.entry((nw, nh)).or_insert(0) += 1;
    }

    let mut min = std::i32::MAX;
    if map.len() < (na + 1) * (nb + 1) {
        min = 0;
    }

    let mut max = std::i32::MIN;
    for (_, v) in map {
        min = min.min(v);
        max = max.max(v);
    }

    println!("{} {}", min, max)
}
