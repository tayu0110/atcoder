use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut uf = UnionFind::new(n);
    for (a, b) in e {
        uf.merge(a - 1, b - 1);
    }

    let r = uf.root(0);
    println!(
        "The graph is {}connected.",
        if (1..n).any(|i| r != uf.root(i)) {
            "not "
        } else {
            ""
        }
    )
}
