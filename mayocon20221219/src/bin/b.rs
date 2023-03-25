#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let mut res = 0;
    for (i, &(x, y)) in p.iter().enumerate() {
        for (j, &(nx, ny)) in p.iter().enumerate().skip(i + 1) {
            for &(px, py) in p.iter().skip(j + 1) {
                let mut v = [(x, y), (nx, ny), (px, py)];
                v.sort();

                let (x, y) = v[0];
                let (px, py) = v[1];
                let (nx, ny) = v[2];

                if x == px && px == nx {
                    continue;
                }

                if y == py && py == ny {
                    continue;
                }

                if (py - y) * (nx - x) != (ny - y) * (px - x) {
                    res += 1;
                }
            }
        }
    }

    println!("{}", res);
}
