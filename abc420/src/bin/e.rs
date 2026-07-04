use std::collections::BTreeSet;

use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, q: usize}

    let mut set = vec![BTreeSet::new(); n];
    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {u: usize, v: usize}
            let (u, v) = (u - 1, v - 1);
            let ur = uf.root(u);
            let vr = uf.root(v);

            if ur != vr {
                uf.merge(ur, vr);
                let new = uf.root(ur);
                if set[ur].len() < set[vr].len() {
                    let s = std::mem::take(&mut set[ur]);
                    set[vr].extend(s.iter());
                    if vr != new {
                        set.swap(vr, new);
                    }
                } else {
                    let s = std::mem::take(&mut set[vr]);
                    set[ur].extend(s.iter());
                    if ur != new {
                        set.swap(ur, new);
                    }
                }
            }
        } else if ty == 2 {
            input! {v: usize}
            let v = v - 1;
            let root = uf.root(v);
            if set[root].contains(&v) {
                set[root].remove(&v);
            } else {
                set[root].insert(v);
            }
        } else {
            input! {v: usize}
            let v = v - 1;
            let root = uf.root(v);
            if !set[root].is_empty() {
                println!("Yes")
            } else {
                println!("No")
            }
        }

        // eprintln!("ty: {ty}, set: {set:?}");
    }
}
