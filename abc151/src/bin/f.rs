use geometry::convex_hull;
use proconio::*;

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    if n == 2 {
        let (s, t) = p[0];
        let (a, b) = p[1];
        println!("{}", ((t - b) as f64).hypot((s - a) as f64) / 2.0);
        return;
    }

    let c = convex_hull(p);
    let len = c.len();
    let mut res = std::f64::MAX;
    let dist = |p: i64, q: i64, s: i64, t: i64| -> f64 { ((p - s) as f64).hypot((q - t) as f64) };
    for i in 0..len {
        for j in i + 1..len {
            for k in j + 1..len {
                let (m, n) = c[i];
                let (p, q) = c[j];
                let (s, t) = c[k];
                let a = dist(p, q, s, t);
                let b = dist(s, t, m, n);
                let c = dist(m, n, p, q);
                let cosa = (b * b + c * c - a * a) / (2.0 * b * c);
                let r = a / (2.0 * (1.0 - cosa * cosa).sqrt());
                if res > r {
                    res = r;
                }
                eprintln!("r: {r}, res: {res:?}");
            }
        }
    }
    println!("{}", res)
}
