use itertools::Itertools;
use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, k: usize, e: [(usize, usize, usize); m]}

    let mut res = usize::MAX;
    for v in (0..m).combinations(n - 1) {
        let mut uf = UnionFind::new(n);
        let mut r = 0;
        for i in v {
            let (u, v, w) = e[i];
            if uf.is_same(u - 1, v - 1) {
                r = usize::MAX;
                break;
            }

            uf.merge(u - 1, v - 1);
            r += w;
            r %= k;
        }

        res = res.min(r);
    }

    println!("{res}")
}
