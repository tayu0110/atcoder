#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn rec(p: i32, a: i32, b: i32, c: i32, na: i32, nb: i32, nc: i32, l: &[i32]) -> i32 {
    if l.len() == 0 {
        if na == 0 || nb == 0 || nc == 0 {
            return std::i32::MAX;
        }
        let res = p + (a - na).abs() + (b - nb).abs() + (c - nc).abs();
        return res;
    }

    let mut res = std::i32::MAX;
    res = std::cmp::min(res, rec(p, a, b, c, na, nb, nc, &l[1..]));
    res = std::cmp::min(
        res,
        rec(
            p + if na != 0 { 10 } else { 0 },
            a,
            b,
            c,
            na + l[0],
            nb,
            nc,
            &l[1..],
        ),
    );
    res = std::cmp::min(
        res,
        rec(
            p + if nb != 0 { 10 } else { 0 },
            a,
            b,
            c,
            na,
            nb + l[0],
            nc,
            &l[1..],
        ),
    );
    res = std::cmp::min(
        res,
        rec(
            p + if nc != 0 { 10 } else { 0 },
            a,
            b,
            c,
            na,
            nb,
            nc + l[0],
            &l[1..],
        ),
    );

    res
}

fn main() {
    input! {n: usize, a: i32, b: i32, c: i32, l: [i32; n]}

    println!("{}", rec(0, a, b, c, 0, 0, 0, &l));
}
