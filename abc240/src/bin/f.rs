#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {t: usize}

    let mut res = vec![];
    for _ in 0..t {
        input! {n: usize, _m: usize, p: [(i64, i64); n]}

        let mut b = vec![0; n+1];
        let mut a = vec![0; n+1];
        let mut r = p[0].0;
        for (i, (x, y)) in p.into_iter().enumerate() {
            b[i+1] = b[i] + x * y;
            a[i+1] = a[i] + b[i] * y + x * (y * (y+1) / 2);
            r = std::cmp::max(r, a[i+1]);

            if b[i] > 0 && b[i+1] < 0 {
                let ny = b[i] / x.abs();
                r = std::cmp::max(r, a[i] + b[i] * ny + x * (ny * (ny+1) / 2));
            } else if b[i] < 0 {
                let ny = 1;
                r = std::cmp::max(r, a[i] + b[i] * ny + x * (ny * (ny+1) / 2));
            }
        }

        res.push(r);
    }

    for res in res {
        println!("{}", res);
    }
}
