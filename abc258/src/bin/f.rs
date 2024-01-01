use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut r = vec![];

    for _ in 0..t {
        input! {b: u64, k: u64, sx: u64, sy: u64, gx: u64, gy: u64}

        let mut res = (sy.abs_diff(gy) + sx.abs_diff(gx)) * k;
        let f = |x: u64, y: u64| -> Vec<(u64, u64, u64)> {
            let (fx, cx) = (x / b * b, (x + b - 1) / b * b);
            let (fy, cy) = (y / b * b, (y + b - 1) / b * b);
            vec![(fx, y), (cx, y), (x, fy), (x, cy)]
                .into_iter()
                .map(|(nx, ny)| (nx, ny, nx.abs_diff(x) + ny.abs_diff(y)))
                .collect()
        };
        let g = |sx: u64, sy: u64, gx: u64, gy: u64| -> u64 {
            let mut res = u64::MAX;
            if sx % b == 0 && gx % b == 0 && sx / b == gx / b {
                res = sy.abs_diff(gy);
            }
            if sy % b == 0 && gy % b == 0 && sy / b == gy / b {
                res = sx.abs_diff(gx);
            }
            for px in vec![sx / b * b, (sx + b - 1) / b * b] {
                for qx in vec![gx / b * b, (gx + b - 1) / b * b] {
                    for py in vec![sy / b * b, (sy + b - 1) / b * b] {
                        for qy in vec![gy / b * b, (gy + b - 1) / b * b] {
                            res = res.min(
                                px.abs_diff(sx)
                                    + qx.abs_diff(gx)
                                    + py.abs_diff(sy)
                                    + qy.abs_diff(gy)
                                    + px.abs_diff(qx)
                                    + py.abs_diff(qy),
                            );
                        }
                    }
                }
            }
            res
        };
        for (sx, sy, ns) in f(sx, sy) {
            for (gx, gy, ng) in f(gx, gy) {
                res = res.min((ns + ng) * k + g(sx, sy, gx, gy));
            }
        }

        r.push(res);
    }

    println!("{}", r.iter().join("\n"))
}
