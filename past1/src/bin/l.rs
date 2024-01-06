use ordered_float::OrderedFloat;
use proconio::*;
use unionfind::UnionFind;

fn solve(t: &[(usize, usize, usize)]) -> f64 {
    let mut e = vec![];
    for (i, &(x, y, c)) in t.iter().enumerate() {
        for (j, &(nx, ny, nc)) in t.iter().enumerate().skip(i + 1) {
            let mut dist = (nx.abs_diff(x) as f64).hypot(ny.abs_diff(y) as f64);
            if c != nc {
                dist *= 10.0;
            }

            e.push((dist, i, j));
        }
    }

    e.sort_unstable_by_key(|e| OrderedFloat::from(e.0));

    let mut uf = UnionFind::new(t.len());
    let mut res = 0.0;
    for (d, i, j) in e {
        if uf.merge(i, j) {
            res += d;
        }
    }
    res
}

fn main() {
    input! {n: usize, m: usize, big: [(usize, usize, usize); n], small: [(usize, usize, usize); m]}

    let mut res = f64::MAX;
    for i in 0..1 << m {
        let mut t = big.clone();
        for j in 0..m {
            if i & (1 << j) != 0 {
                t.push(small[j]);
            }
        }

        res = res.min(solve(&t));
    }

    println!("{res}");
}
