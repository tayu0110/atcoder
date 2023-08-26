use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut has_edge = HashSet::new();
    let mut edges = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {u: usize, v: usize}
            edges[u - 1].insert(v - 1);
            edges[v - 1].insert(u - 1);
            has_edge.insert(u - 1);
            has_edge.insert(v - 1);
        } else {
            input! {v: usize}

            let mut k = vec![];
            for &to in &edges[v - 1] {
                k.push(to);
            }

            for to in k {
                edges[to].remove(&(v - 1));

                if edges[to].is_empty() {
                    has_edge.remove(&to);
                }
            }

            edges[v - 1].clear();
            has_edge.remove(&(v - 1));
        }

        println!("{}", n - has_edge.len())
    }
}
