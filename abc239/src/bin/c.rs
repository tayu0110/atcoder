#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {x: i64, y: i64, c: i64, r: i64}

    for i in -5..=5 {
        for j in -5..=5 {
            if i*i + j*j == 5 {
                let (nx, ny) = (x + i, y + j);
                if (c - nx) * (c - nx) + (r - ny) * (r - ny) == 5 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
