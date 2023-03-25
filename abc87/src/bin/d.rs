use proconio::*;
use unionfind::WeightedUnionFind;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, i64); m]}

    let mut uf = WeightedUnionFind::new(n);
    for (l, r, d) in p {
        if uf.is_same(l - 1, r - 1) {
            if uf.diff(l - 1, r - 1) != d {
                println!("No");
                return;
            }
        } else {
            uf.merge(l - 1, r - 1, d).unwrap();
        }
    }

    println!("Yes")
}
