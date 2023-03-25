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

fn lcm(x: usize, y: usize) -> usize {
    (x / gcd(x, y)).saturating_mul(y)
}

#[fastout]
fn main() {
    input! {n: usize, m: usize, mut a: [usize; n]}

    a.iter_mut().for_each(|c| *c /= 2);

    let l = a.iter().fold(1, |s, v| lcm(s, *v));

    if l > m {
        println!("0");
        return;
    }

    if a.iter().any(|c| (l / *c) % 2 == 0) {
        println!("0");
        return;
    }

    println!("{}", (m / l + 1) / 2);
}
