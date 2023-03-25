#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}

    let (mut l, mut r) = (0, 1000_000_010);
    while r - l > 1 {
        let m = (r + l) / 2;
        if m * m <= n {
            l = m;
        } else {
            r = m;
        }
    }

    let rem = n - l * l;

    if rem == 0 {
        println!("{}", l);
    } else {
        let l = l + 1;
        if rem <= l {
            println!("{}", l - (rem - 1));
        } else {
            println!("{}", rem - (l - 1));
        }
    }
}
