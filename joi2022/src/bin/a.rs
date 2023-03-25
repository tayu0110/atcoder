#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, mut a: [usize; n], q: usize, x: [usize; q]}

    let mut v = vec![1; n];
    for i in 0..n {
        while a[i] % 2 == 0 {
            a[i] /= 2;
            v[i] *= 2;
        }
    }

    for i in 1..n {
        v[i] += v[i - 1];
    }

    for x in x {
        let (mut l, mut r) = (-1, n as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if x <= v[m as usize] {
                r = m;
            } else {
                l = m;
            }
        }

        println!("{}", a[r as usize]);
    }
}
