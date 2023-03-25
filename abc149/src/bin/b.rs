#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {mut a: usize, mut b: usize, mut k: usize}

    if a > k {
        a -= k;
        k = 0;
    } else {
        k -= a;
        a = 0;
    }

    if b > k {
        b -= k;
    } else {
        b = 0;
    }

    println!("{} {}", a, b);
}
