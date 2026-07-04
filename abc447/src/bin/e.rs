use math::MathInt;
use proconio::*;
use unionfind::UnionFind;

const M: usize = 998244353;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut uf = UnionFind::new(n);
    let mut t = n;
    let mut ret = 0;
    for (i, (u, v)) in e.into_iter().enumerate().rev() {
        if uf.is_same(u - 1, v - 1) {
            continue;
        }

        if t == 2 && uf.root(u - 1) != uf.root(v - 1) {
            ret += 2usize.pow_mod(i as u64 + 1, M);
            ret %= M;
            continue;
        }

        if uf.root(u - 1) != uf.root(v - 1) {
            t -= 1;
        }
        uf.merge(u - 1, v - 1);
    }

    println!("{ret}")
}
