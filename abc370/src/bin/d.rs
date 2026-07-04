use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, q: usize, query: [(usize, usize); q]}

    let mut row = vec![(1..=w).collect::<BTreeSet<_>>(); h + 1];
    let mut col = vec![(1..=h).collect::<BTreeSet<_>>(); w + 1];

    for (r, c) in query {
        if row[r].contains(&c) {
            row[r].remove(&c);
            col[c].remove(&r);
        } else {
            if let Some(&c) = row[r].range(..c).next_back() {
                row[r].remove(&c);
                col[c].remove(&r);
            }
            if let Some(&c) = row[r].range(c + 1..).next() {
                row[r].remove(&c);
                col[c].remove(&r);
            }
            if let Some(&r) = col[c].range(..r).next_back() {
                row[r].remove(&c);
                col[c].remove(&r);
            }
            if let Some(&r) = col[c].range(r + 1..).next() {
                row[r].remove(&c);
                col[c].remove(&r);
            }
        }
    }

    row.remove(0);
    col.remove(0);
    println!("{}", row.into_iter().map(|s| s.len()).sum::<usize>())
}
