#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn solve(x: i64, y: i64, a: i64, b: i64, c: i64) -> bool {
    let ya = (a + x - 1) / x;
    let y = y - ya;

    if y <= 0 {
        return false;
    }

    let yb = (b + x - 1) / x;
    let yc = (c + x - 1) / x;
    let xb = (b + y - 1) / y;
    let xc = (c + y - 1) / y;
    yb + yc <= y || xb + xc <= x
}

fn main() {
    input! {x: i64, y: i64, a: i64, b: i64, c: i64}
    let mut v = [a, b, c];

    for _ in 0..3 {
        if solve(x, y, v[0], v[1], v[2]) {
            println!("Yes");
            return;
        }
        if solve(y, x, v[0], v[1], v[2]) {
            println!("Yes");
            return;
        }

        v.rotate_left(1);
    }

    println!("No");
}
