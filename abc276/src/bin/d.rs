#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

#[fastout]
fn main() {
    input! {n: usize, mut a: [usize; n]}

    let g = a.iter().fold(0, |s, v| gcd(s, *v));

    let mut res = 0;
    for i in 0..n {
        a[i] /= g;

        while a[i] % 2 == 0 {
            a[i] /= 2;
            res += 1;
        }
        while a[i] % 3 == 0 {
            a[i] /= 3;
            res += 1;
        }

        if a[i] != 1 {
            println!("-1");
            return;
        }
    }

    println!("{}", res);
}
