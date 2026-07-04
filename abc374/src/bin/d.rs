use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, s: f64, t: f64, p: [(f64, f64, f64, f64); n]}

    let mut res = f64::MAX;
    for i in 0..1 << n {
        let mut v = vec![];
        for j in 0..n {
            if i & (1 << j) == 0 {
                v.push(p[j]);
            } else {
                v.push((p[j].2, p[j].3, p[j].0, p[j].1));
            }
        }

        for x in (0..n).permutations(n) {
            let mut sum = 0.0;
            let mut w = vec![(0.0, 0.0)];
            for x in x {
                w.push((v[x].0, v[x].1));
                w.push((v[x].2, v[x].3));
            }

            for (i, chunk) in w.windows(2).enumerate() {
                let (x, y) = chunk[0];
                let (nx, ny) = chunk[1];

                let dist = (ny - y).hypot(nx - x);
                if i % 2 == 0 {
                    sum += dist / s;
                } else {
                    sum += dist / t;
                }
            }

            res = res.min(sum);
        }
    }

    println!("{res}")
}
