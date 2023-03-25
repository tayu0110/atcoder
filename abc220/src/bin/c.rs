#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, a: [usize; n], x: usize}

    let sum: usize = a.iter().sum();

    let mut res = x / sum;
    let mut x = x - res * sum;
    res *= n;

    for a in a {
        res += 1;
        if x < a {
            break;
        }

        x -= a;
    }

    println!("{}", res);
}
