use itertools::Itertools;
use proconio::*;
use unionfind::WeightedUnionFind;

fn main() {
    input! {n: usize, q: usize, e: [(usize, usize, i64); q]}

    let mut uf = WeightedUnionFind::new(n);
    let mut res = vec![];
    for (i, (a, b, d)) in e.into_iter().enumerate() {
        if uf.is_same(a - 1, b - 1) {
            if uf.diff(a - 1, b - 1) == d {
                res.push(i + 1);
            }
        } else {
            let r = uf.merge(a - 1, b - 1, d);
            if r.is_ok() {
                res.push(i + 1);
            }
        }
    }

    println!("{}", res.iter().join(" "))
}
