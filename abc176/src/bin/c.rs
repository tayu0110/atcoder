#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut now = 0;
    let mut res = 0;
    for a in a {
        if now > a {
            res += now - a;
        } else {
            now = a;
        }
    }

    println!("{}", res);
}
