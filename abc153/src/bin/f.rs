#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, d: usize, a: i64, mut p: [(usize, i64); n]}
    p.sort();

    let mut nt = std::collections::VecDeque::new();
    let mut res = 0;
    let mut damages = 0;
    for (x, mut h) in p {
        while let Some((k, d)) = nt.pop_front() {
            if k < x {
                damages -= d;
            } else {
                nt.push_front((k, d));
                break;
            }
        }

        h -= damages;
        if h <= 0 {
            continue;
        }

        let k = (h + a - 1) / a;
        damages += k * a;
        nt.push_back((x + 2*d, k * a));
        res += k;
    }

    println!("{}", res);
}
