use math::factorize;
use proconio::*;

fn main() {
    input! {w: u64, d: usize, n: usize, a: [[u64; n]; d]}

    for a in a {
        let mut sum = a.iter().sum::<u64>();
        let mut a = a.into_iter().enumerate().collect::<Vec<_>>();
        a.sort_unstable_by_key(|v| v.1);

        let mut res = vec![(0, 0, 0, 0); n];
        let mut t = vec![(w as u64, w as u64, 0, 0)];
        let mut rem = w * w;
        while let Some((i, a)) = a.pop() {
            sum -= a;
            let mut factor = factorize(a as u64);
            factor.sort_unstable();

            let mut diff = u64::MAX;
            let mut j = None;
            for k in 0..t.len() {
                let (h, w, _, _) = t[k];
                if h * w < a {
                    continue;
                }

                {
                    let fp = factor.partition_point(|&f| f <= h);
                    if fp > 0
                        && factor[fp - 1] <= h
                        && a / factor[fp - 1] <= w
                        && h - factor[fp - 1] < diff
                    {
                        diff = h - factor[fp - 1];
                        j = Some((k, true));
                    }
                }
                {
                    let fp = factor.partition_point(|&f| f <= w);
                    if fp > 0
                        && factor[fp - 1] <= w
                        && a / factor[fp - 1] <= h
                        && w - factor[fp - 1] < diff
                    {
                        diff = w - factor[fp - 1];
                        j = Some((k, false));
                    }
                }
            }

            if let Some((j, f)) = j {
                if f {
                    let (h, w, r, c) = t.swap_remove(j);
                    let fp = factor.partition_point(|&f| f <= h) - 1;
                    let f = factor[fp];
                    let g = a / f;
                    res[i] = (r, c, r + f, c + g);
                    rem -= a;
                    if h != f {
                        t.push((h - f, w, r + f, c));
                    }
                    if w != g {
                        t.push((f, w - g, r, c + g));
                    }
                } else {
                    let (h, w, r, c) = t.swap_remove(j);
                    let fp = factor.partition_point(|&f| f <= w) - 1;
                    let g = factor[fp];
                    let f = a / g;
                    rem -= a;
                    res[i] = (r, c, r + f, c + g);
                    if h != f {
                        t.push((h - f, g, r + f, c));
                    }
                    if w != g {
                        t.push((h, w - g, r, c + g));
                    }
                }
            } else {
                let max = t.iter().map(|(h, w, _, _)| h * w).max().unwrap();
                let pos = t.iter().position(|(h, w, _, _)| h * w == max).unwrap();
                let (h, w, r, c) = t.swap_remove(pos);
                let rhh = h * ((a + h - 1) / h).min(w).max(1);
                let rhw = w * ((a + w - 1) / w).min(h).max(1);
                if (1..=w).contains(&((a + h - 1) / h))
                    && (rhh.abs_diff(a) < rhw.abs_diff(a) || !(1..=h).contains(&((a + w - 1) / w)))
                    && rem >= rhh + sum
                {
                    res[i] = (r, c, r + h, c + ((a + h - 1) / h).min(w).max(1));
                    if w > (a + h - 1) / h {
                        t.push((h, w - (a + h - 1) / h, r, c + (a + h - 1) / h));
                    }
                    rem -= rhh;
                } else if (1..=h).contains(&((a + w - 1) / w))
                    && (rhh.abs_diff(a) > rhw.abs_diff(a) || !(1..=w).contains(&((a + h - 1) / h)))
                    && rem >= rhw + sum
                {
                    res[i] = (r, c, r + ((a + w - 1) / w).min(h).max(1), c + w);
                    if h > a / w {
                        t.push((h - (a + w - 1) / w, w, r + (a + w - 1) / w, c));
                    }
                    rem -= rhw;
                } else if a - h * (a / h).min(w).max(1) < a - w * (a / w).min(h).max(1) {
                    res[i] = (r, c, r + h, c + (a / h).min(w).max(1));
                    if w > a / h {
                        t.push((h, w - a / h, r, c + a / h));
                    }
                    rem -= h * (a / h).min(w).max(1);
                } else {
                    res[i] = (r, c, r + (a / w).min(h).max(1), c + w);
                    if h > a / w {
                        t.push((h - a / w, w, r + a / w, c));
                    }
                    rem -= w * (a / w).min(h).max(1);
                }
            }
        }

        for (r, c, h, w) in res {
            println!("{r} {c} {h} {w}");
        }
    }
}
