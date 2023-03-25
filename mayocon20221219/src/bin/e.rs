#![allow(unused_imports)]
use matrix::AffineTransformation;
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, p: [(i64, i64); n], m: usize}

    let mut op = vec![AffineTransformation::<i64>::new()];
    for _ in 0..m {
        input! {t: usize}

        let nop = op[op.len() - 1].clone();
        if t == 1 {
            op.push(nop.rotate_clockwise());
        } else if t == 2 {
            op.push(nop.rotate_counterclockwise());
        } else if t == 3 {
            input! {p: i64}
            op.push(
                nop.translation(-p, 0)
                    .reflection(true, false)
                    .translation(p, 0),
            );
        } else {
            input! {p: i64}
            op.push(
                nop.translation(0, -p)
                    .reflection(false, true)
                    .translation(0, p),
            );
        }
    }

    input! {q: usize, q: [(usize, usize); q]}
    for (a, b) in q {
        let (x, y) = p[b - 1];
        let (x, y) = op[a].transform(x, y);

        println!("{} {}", x, y);
    }
}
