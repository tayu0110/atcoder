#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, mut k: usize, x: usize, mut a: [usize; n]}

    for i in 0..n {
        let v = a[i] / x;
        if v >= k {
            a[i] -= k * x;
            k = 0;
        } else {
            a[i] -= v * x;
            k -= v;
        }
    }

    a.sort_by_key(|v| std::cmp::Reverse(*v));

    for v in a.iter_mut().take(k) {
        *v = 0;
    }

    println!("{}", a.iter().sum::<usize>());
}
