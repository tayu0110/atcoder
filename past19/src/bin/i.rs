use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, mut e: [(u32, u32, u32); m]}
    e.sort_unstable_by_key(|e| e.2);

    let mut uf = UnionFind::new(n + 1);
    while let Some((a, b, d)) = e.pop() {
        uf.merge(a as usize, b as usize);
        if uf.size(a as usize) == n {
            println!("{}", d);
            return;
        }
    }
}
