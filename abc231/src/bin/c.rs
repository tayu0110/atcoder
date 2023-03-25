#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, q: usize, mut a: [usize; n], x: [usize; q]}
    a.sort();

    for x in x {
        let (mut l, mut r) = (-1, n as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if a[m as usize] < x {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{}", n - r as usize);
    }
}
