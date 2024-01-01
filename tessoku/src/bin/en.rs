use proconio::*;
use std::cmp::Reverse;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, mut e: [(usize, usize, u32); m]}

    e.sort_unstable_by_key(|v| Reverse(v.2));

    let mut uf = UnionFind::new(n);
    let mut res = 0;
    for (a, b, c) in e {
        if !uf.is_same(a - 1, b - 1) {
            uf.merge(a - 1, b - 1);
            res += c;
        }
    }

    println!("{res}")
}
