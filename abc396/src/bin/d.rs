use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v, w) in e {
        t[u - 1].push((v - 1, w));
        t[v - 1].push((u - 1, w));
    }

    let mut checked = FxHashSet::default();
    let mut nt = vec![];
    nt.push((1, 0, 0));
    let mut res = usize::MAX;
    while let Some((set, now, xor)) = nt.pop() {
        if now == n - 1 {
            res = res.min(xor);
            continue;
        }
        for &(to, w) in &t[now] {
            if set & (1 << to) != 0 {
                continue;
            }
            let next = set | (1 << to);
            if checked.insert((next, xor ^ w)) {
                nt.push((next, to, xor ^ w));
            }
        }
    }

    println!("{res}")
}
