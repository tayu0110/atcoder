use proconio::*;

fn main() {
    input! {n: usize, h: usize, p: [(usize, usize); n]}

    let (mut l, mut r) = (0, 1000_000_000_000_000_100usize);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut ok = vec![];
        let mut max_d = 0;

        for &(t, d) in &p {
            if t <= m {
                ok.push((t * d, t, d));
            } else {
                max_d = max_d.max(d);
            }
        }

        ok.sort();

        let mut rem = m;
        let mut dm = 0;
        while let Some((damage, t, d)) = ok.pop() {
            if t > rem {
                max_d = max_d.max(d);
                continue;
            }

            if rem * max_d <= damage {
                dm += damage;
                rem -= t;
            } else {
                max_d = max_d.max(d);
            }
        }

        if dm >= h {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", r)
}
