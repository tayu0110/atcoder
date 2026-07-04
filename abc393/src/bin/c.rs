use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {_: usize, m: usize, e: [(usize, usize); m]}

    let mut edges = FxHashSet::default();
    let mut res = 0;
    for (u, v) in e {
        if u == v {
            res += 1;
            continue;
        }

        let (u, v) = (u.min(v), u.max(v));
        if edges.contains(&(u, v)) {
            res += 1;
            continue;
        }

        edges.insert((u, v));
    }

    println!("{res}")
}
