#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h]}

    for i1 in 0..h {
        for j1 in 0..w {
            for i2 in i1..h {
                for j2 in j1..w {
                    if a[i1][j1] + a[i2][j2] > a[i2][j1] + a[i1][j2] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }

    println!("Yes");
}
