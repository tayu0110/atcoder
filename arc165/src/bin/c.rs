use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, mut e: [(usize, usize, usize); m]}
    e.iter_mut().for_each(|v| {
        v.0 -= 1;
        v.1 -= 1;
    });
    e.sort_by_key(|v| v.2);

    let mut res = usize::MAX;
    let mut op: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    for &(a, b, w) in &e {
        if uf.is_same(a, b) {
            res = res.min(w);
        } else {
            if !op[a].is_empty() {
                uf.merge(op[a][0].0, b);
            }
            if !op[b].is_empty() {
                uf.merge(op[b][0].0, a);
            }
            op[a].push((b, w));
            op[b].push((a, w));
        }
    }

    for v in op {
        if v.len() < 2 {
            continue;
        }
        let mut v = v.into_iter().map(|v| v.1).collect::<Vec<_>>();
        v.sort();

        res = res.min(v[0] + v[1]);
    }

    println!("{}", res)
}
