#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize}

    let mut k = vec![];
    for _ in 0..m {
        input! {c: usize, a: [usize; c]}

        let mut l = 0;
        for a in a {
            l |= 1 << (a - 1);
        }

        k.push(l);
    }

    let mut res = 0;
    for i in 1..(1 << m) {
        let mut t = 0;
        for j in 0..m {
            if i & (1 << j) != 0 {
                t |= k[j];
            }
        }

        if t == (1 << n) - 1 {
            res += 1;
        }
    }

    println!("{}", res);
}
