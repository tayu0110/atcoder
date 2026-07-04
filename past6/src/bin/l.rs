use ordered_float::OrderedFloat;
use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, p: [(f64, f64); n], c: [(f64, f64, f64); m]}

    let mut edges = vec![];
    for (i, (px, py)) in p.iter().enumerate() {
        for (j, (cx, cy)) in p.iter().enumerate().skip(i + 1) {
            edges.push((i, j, (cx - px).hypot(cy - py)));
        }
    }

    let mut circles = [[0.0; 8]; 8];
    for (i, &(px, py, r)) in c.iter().enumerate() {
        for (j, &(cx, cy, s)) in c.iter().enumerate().skip(i + 1) {
            let d2 = (px - cx).powi(2) + (py - cy).powi(2);
            circles[i][j] = if (r - s).powi(2) <= d2 && d2 <= (r + s).powi(2) {
                -1.0
            } else if d2 < (r - s).powi(2) {
                r.max(s) - r.min(s) - d2.sqrt()
            } else {
                d2.sqrt() - r - s
            };
            circles[j][i] = circles[i][j];
        }
    }
    let mut res = f64::MAX;
    for i in 0..1 << m {
        let mut edges = edges.clone();
        let mut uf = UnionFind::new(n + m);
        for j in 0..m {
            if i & (1 << j) != 0 {
                let (cx, cy, r) = c[j];
                for (k, &(px, py)) in p.iter().enumerate() {
                    edges.push((k, j + n, (r - (px - cx).hypot(py - cy)).abs()));
                }
                for k in 0..j {
                    if i & (1 << k) != 0 {
                        if circles[j][k] < 0.0 {
                            uf.merge(n + j, n + k);
                        } else {
                            edges.push((n + j, n + k, circles[j][k]));
                        }
                    }
                }
            }
        }
        let mut sum = 0.0;
        edges.sort_unstable_by_key(|v| OrderedFloat::from(v.2));
        for (i, j, d) in edges {
            if !uf.is_same(i, j) {
                uf.merge(i, j);
                sum += d;
            }
        }
        res = res.min(sum);
    }
    println!("{res}")
}
