#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, mut t: Chars}

    let (mut x, mut y) = (0i32, 0i32);
    let mut i = 0;
    for (dx, dy) in vec![(1, 0), (0, -1), (-1, 0), (0, 1)].into_iter().cycle() {
        while i < n {
            if t[i] == 'S' {
                x += dx;
                y += dy;
                i += 1;
                continue;
            }
            i += 1;
            break;
        }
        if i == n {
            break;
        }
    }

    println!("{} {}", x, y);
}
