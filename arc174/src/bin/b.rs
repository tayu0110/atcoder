use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut res = vec![];
    for _ in 0..t {
        input! {a: [usize; 5], p: [usize; 5]}

        let w = a
            .iter()
            .enumerate()
            .map(|(i, a)| a * (i + 1))
            .sum::<usize>();
        let sum = a.iter().sum::<usize>();

        if w >= 3 * sum {
            res.push(0);
            continue;
        }

        let s = 3 * sum - w;
        let (mut l, mut r) = (0, s + 10);
        while r - l > 2 {
            let (y1, y2) = ((l * 2 + r) / 3, (l + r * 2) / 3);
            let res1 = {
                let x1 = (s.saturating_sub(y1) + 1) / 2;
                p[4] * x1 + p[3] * y1
            };
            let res2 = {
                let x2 = (s.saturating_sub(y2) + 1) / 2;
                p[4] * x2 + p[3] * y2
            };

            if res1 < res2 {
                r = y2;
            } else {
                l = y1;
            }
        }

        let mut r = {
            let y = 0;
            let x = (s + 1) / 2;
            p[4] * x + p[3] * y
        };
        for y in 0..10 {
            let x = (s.saturating_sub(y) + 1) / 2;
            r = r.min(p[4] * x + p[3] * y);
        }
        for y in s.saturating_sub(10)..=s {
            let x = (s.saturating_sub(y) + 1) / 2;
            r = r.min(p[4] * x + p[3] * y);
        }
        for y in l.saturating_sub(10)..l + 10 {
            let x = (s.saturating_sub(y) + 1) / 2;
            r = r.min(p[4] * x + p[3] * y);
        }
        res.push(r);
    }

    println!("{}", res.iter().join("\n"))
}
