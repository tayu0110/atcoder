use proconio::*;
use rustc_hash::{FxHashMap, FxHashSet};
use unionfind::UnionFind;

type HashSet<S> = FxHashSet<S>;
type HashMap<K, V> = FxHashMap<K, V>;

fn solve(points: &[(usize, usize)]) -> usize {}

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut rev = HashMap::default();
    let mut uf = UnionFind::new(n);
    for (i, &(x, y)) in p.iter().enumerate() {
        rev.insert((x, y), i);

        for i in [0, 1, !0] {
            for j in [0, 1, !0] {
                let dx = x.wrapping_add(i);
                let dy = y.wrapping_add(j);
                if dx == x && dy == y {
                    continue;
                }

                if let Some(&index) = rev.get(&(dx, dy)) {
                    uf.merge(i, index);
                }
            }
        }
    }

    let mut groups = HashMap::default();
    for i in 0..n {
        let root = uf.root(i);
        groups.entry(root).or_insert(vec![]).push(p[i]);
    }

    let mut res = 0;
    for g in groups.values() {
        res += solve(g.as_slice());
    }

    println!("{res}");
}
