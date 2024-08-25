use std::ops::{Add, Mul};

use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize}

    let add = &Add::add as &dyn Fn(usize, usize) -> usize;
    let mul = &Mul::mul as &dyn Fn(usize, usize) -> usize;

    let mut res = 0;
    for (f, g) in [(add, add), (add, mul), (mul, add), (mul, mul)] {
        res = res.max(g(f(a, b), c));
    }

    println!("{}", res);
}
