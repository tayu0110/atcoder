use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut res = vec![];
    for _ in 0..t {
        input! {mut a: usize, mut b: usize, mut c: usize}

        let min = a.min(b).min(c);
        a -= min;
        c -= min;

        if a == 0 || c == 0 {
            res.push(min);
            continue;
        }

        let (mut l, mut r) = (0, 10000000000);
        while r - l > 1 {
            let m = (r + l) / 2;
            if a < m || c < m {
                r = m;
                continue;
            }

            let rem = a - m + c - m;
            if rem >= m {
                l = m;
            } else {
                r = m;
            }
        }

        res.push(min + l);
    }

    println!("{}", res.iter().join("\n"))
}
