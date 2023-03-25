#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; m]}

    let mut sum = 0;
    for i in 0..n {
        if b.contains(&(i + 1)) {
            sum += a[i];
        }
    }

    println!("{}", sum);
}
