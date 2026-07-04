use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}
    let mut uf = UnionFind::new(n + 1);
    println!(
        "{}",
        e.into_iter().filter(move |&(u, v)| !uf.merge(u, v)).count()
    );
}
