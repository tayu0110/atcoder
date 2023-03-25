#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, p: [(usize, usize); n]}
    let len = p.iter().cloned().fold(0, |s, (v, _)| s + v);

    let (mut l, mut r) = (0.0, 1e9);
    let mut res = 0.0;
    while r - l > 1e-8 {
        let (mut ml, mut mr) = ((r+l) / 2.0, (r+l) / 2.0);
        let (mut ld, mut rd) = (0.0, 0.0);
        for &(a, b) in &p {
            if b as f64 * ml > a as f64 {
                ml -= a as f64 / b as f64;
                ld += a as f64;
            } else {
                ld += b as f64 * ml;
                break;
            }
        }
        res = ld;
        for &(a, b) in p.iter().rev() {
            if b as f64 * mr > a as f64 {
                mr -= a as f64 / b as f64;
                rd += a as f64;
            } else {
                rd += b as f64 * mr;
                break;
            }
        }

        if ld + rd > len as f64 {
            r = (r+l) / 2.0;
        } else {
            l = (r+l) / 2.0;
        }
    }

    println!("{}", res);
}
