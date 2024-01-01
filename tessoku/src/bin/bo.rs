use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, mut p: [(u32, u32, u16); m]}
    p.sort_unstable_by_key(|v| v.2);

    let mut uf = UnionFind::new(n);
    let mut res = 0u32;
    for (a, b, c) in p {
        if !uf.is_same(a as usize - 1, b as usize - 1) {
            uf.merge(a as usize - 1, b as usize - 1);
            res += c as u32;
        }
    }

    println!("{res}")
}
