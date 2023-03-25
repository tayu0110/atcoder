#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};
use string::zalgorithm;

fn main() {
    input! {n: usize, t: Chars}

    let s_sinv_c = t.iter().chain(t.iter().rev()).collect::<String>();
    let sinv_s_c = t.iter().rev().chain(t.iter()).collect::<String>();

    let s_sinv = zalgorithm(s_sinv_c);
    let sinv_s = zalgorithm(sinv_s_c);

    for (i, &z) in s_sinv.iter().enumerate().skip(2 * n).take(n) {
        if z == 0 {
            continue;
        }

        let b = i - 2 * n;
        if z + b < n {
            continue;
        }

        let f = n - b;
        if sinv_s[2 * n + f] >= b {
            let res = t
                .iter()
                .take(f)
                .chain(t.iter().skip(f + n).take(b))
                .collect::<String>();
            println!("{} {}", res, f);
            return;
        }
    }

    println!("-1");
}
