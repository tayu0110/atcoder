use proconio::*;
use std::collections::BTreeSet;

const MAX: usize = 200001;

fn main() {
    input! {n: usize, q: usize, p: [(usize, usize); n], q: [(usize, usize); q]}

    let mut s = vec![0; n];
    let mut class = vec![0; n];
    let mut t = vec![BTreeSet::new(); MAX];
    let mut u = vec![std::usize::MAX; MAX];
    for (child, (a, b)) in p.into_iter().enumerate() {
        t[b].insert((a, child));
        class[child] = b;
        s[child] = a;
    }

    let mut set = BTreeSet::new();
    for class in 0..MAX {
        if let Some(&(b, nc)) = t[class].iter().next_back() {
            set.insert((b, class, nc));
            u[class] = b;
        }
    }

    for (child, d) in q.into_iter().map(|(c, d)| (c - 1, d)) {
        t[class[child]].remove(&(s[child], child));
        t[d].insert((s[child], child));
        for class in [class[child], d] {
            if let Some(&(b, child)) = t[class].iter().next_back() {
                set.insert((b, class, child));
                u[class] = b;
            } else {
                u[class] = std::usize::MAX;
            }
        }
        class[child] = d;

        while let Some(&(b, nc, child)) = set.iter().next() {
            if nc != class[child] || u[nc] != b {
                set.remove(&(b, nc, child));
                continue;
            }

            println!("{}", b);
            break;
        }
    }
}
