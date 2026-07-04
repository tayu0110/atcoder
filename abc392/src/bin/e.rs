use proconio::*;
use rustc_hash::FxHashSet;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, cables: [(usize, usize); m]}

    let mut uf = UnionFind::new(n);
    let mut rem = vec![];
    for (i, (a, b)) in cables.into_iter().enumerate() {
        if !uf.is_same(a - 1, b - 1) {
            uf.merge(a - 1, b - 1);
        } else {
            rem.push((i, a - 1, b - 1));
        }
    }

    let mut readers = FxHashSet::default();
    for i in 0..n {
        readers.insert(uf.root(i));
    }

    let mut res = vec![];
    for (i, a, b) in rem {
        let ra = uf.root(a);
        let mut found = None;
        for r in readers.iter().take(2) {
            if *r != ra {
                res.push((i + 1, b + 1, *r + 1));
                found = Some(*r);
                break;
            }
        }

        if let Some(found) = found {
            readers.remove(&found);
            readers.remove(&ra);
            uf.merge(ra, found);
            readers.insert(uf.root(ra));
        }
    }

    println!("{}", res.len());
    for (i, j, k) in res {
        println!("{i} {j} {k}")
    }
}
