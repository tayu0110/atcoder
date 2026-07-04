use std::fmt::Write as _;

use math::MathInt;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut buf = String::new();
    for _ in 0..t {
        input! {mut c: usize, d: usize}

        let cs = c.to_string();
        let mut ret = 0;
        let mut ten = 1;
        for _ in 0..11 {
            ten *= 10;

            let min = c * ten;
            let max = (c + 1) * ten - 1;

            let mut sq_min = min.sqrti();
            if sq_min * sq_min < min {
                sq_min += 1;
            }
            let sq_max = max.sqrti();

            if sq_max < sq_min {
                continue;
            }

            if (sq_min * sq_min)
                .to_string()
                .strip_prefix(&cs)
                .and_then(|suf| suf.parse::<usize>().ok())
                .unwrap_or(0)
                > c + d
            {
                continue;
            }
            if (sq_max * sq_max)
                .to_string()
                .strip_prefix(&cs)
                .and_then(|suf| suf.parse::<usize>().ok())
                .unwrap_or(0)
                <= c
            {
                continue;
            }

            let (mut l, mut r) = (sq_min, sq_max + 1);
            while r - l > 1 {
                let m = (r + l) / 2;
                let sq = (m * m).to_string();
                let suffix = sq.strip_prefix(&cs).unwrap();
                if suffix.starts_with('0') {
                    l = m;
                    continue;
                }

                let suffix = suffix.parse::<usize>().unwrap_or(0);
                if suffix <= c {
                    l = m;
                } else {
                    r = m;
                }
            }

            let min = if l == sq_max {
                let sq = (l * l).to_string();
                let suffix = sq.strip_prefix(&cs).unwrap();
                if suffix.starts_with('0') {
                    continue;
                }
                let suffix = suffix.parse::<usize>().unwrap_or(0);
                if suffix <= c {
                    continue;
                }
                l
            } else {
                let sq = (l * l).to_string();
                let suffix = sq.strip_prefix(&cs).unwrap();
                if suffix.starts_with('0') {
                    r
                } else {
                    let suffix = suffix.parse::<usize>().unwrap_or(0);
                    if suffix <= c {
                        r
                    } else {
                        l
                    }
                }
            };

            let (mut l, mut r) = (min, sq_max + 1);
            while r - l > 1 {
                let m = (r + l) / 2;
                let sq = (m * m).to_string();
                let suffix = sq
                    .strip_prefix(&cs)
                    .and_then(|suf| suf.parse::<usize>().ok())
                    .unwrap_or(0);
                if suffix > c + d {
                    r = m;
                } else {
                    l = m;
                }
            }

            if ((r - 1) * (r - 1))
                .to_string()
                .strip_prefix(&cs)
                .and_then(|suf| suf.parse::<usize>().ok())
                .unwrap_or(0)
                > c + d
            {
                continue;
            }

            // eprintln!(
            //     "c: {c}, d: {d}, ten: {ten}, min: {min}, r: {r}, min^2: {}, r^2: {}",
            //     min * min,
            //     (r - 1) * (r - 1)
            // );
            // eprintln!(
            //     "max_diff: {}",
            //     ((r - 1) * (r - 1))
            //         .to_string()
            //         .strip_prefix(&cs)
            //         .and_then(|suf| suf.parse::<usize>().ok())
            //         .unwrap_or(0)
            //         - c
            // );
            ret += r - min;
        }

        writeln!(buf, "{ret}").unwrap();
    }

    print!("{buf}")
}
