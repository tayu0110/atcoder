use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut pone = FxHashSet::default();
    let mut ret = 0;
    let mut row = vec![vec![]; n + 1];
    let mut col = vec![vec![]; n + 1];
    for (r, c) in e {
        for c in row[r].drain(..) {
            if pone.remove(&(r, c)) {
                ret -= 1;
            }
        }
        for r in col[c].drain(..) {
            if pone.remove(&(r, c)) {
                ret -= 1;
            }
        }
        row[r].push(c);
        col[c].push(r);
        pone.insert((r, c));
        ret += 1;
    }

    println!("{ret}")
}
