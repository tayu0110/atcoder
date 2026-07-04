use proconio::*;
use rustc_hash::FxHashMap;
use unionfind::UnionFind;

fn solve(_: usize, w: usize, p: &[(usize, usize)]) -> bool {
    // p.len(): left side, p.len() + 1: other
    let mut uf = UnionFind::new(p.len() + 2);
    let mut map = FxHashMap::default();
    for (i, &(r, c)) in p.iter().enumerate() {
        map.insert((r, c), i);
    }

    for (i, &(r, c)) in p.iter().enumerate() {
        if c == 1 {
            uf.merge(i, p.len());
        } else if r == 1 || c == w {
            uf.merge(i, p.len() + 1);
        }
        for (dr, dc) in [
            (0, 1),
            (1, 0),
            (0, !0),
            (!0, 0),
            (1, 1),
            (1, !0),
            (!0, 1),
            (!0, !0),
        ] {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if let Some(&index) = map.get(&(nr, nc)) {
                uf.merge(i, index);
            }
        }
    }

    uf.is_same(p.len(), p.len() + 1)
}

fn main() {
    input! {h: usize, w: usize, k: usize, mut p: [(usize, usize); k]}

    if h == 1 || w == 1 {
        if p.is_empty() {
            println!("Yes")
        } else {
            println!("No");
        }
        return;
    }

    if solve(h, w, &p) {
        println!("No");
        return;
    }

    p.iter_mut().for_each(|p| *p = (w + 1 - p.1, h + 1 - p.0));
    if solve(w, h, &p) {
        println!("No")
    } else {
        println!("Yes")
    }
}
