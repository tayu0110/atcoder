#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, mut p: [usize; n]}

    for i in (0..p.len()-1).rev() {
        if p[i] < p[i+1] {
            continue;
        }

        for j in (i+1..p.len()).rev() {
            if p[i] > p[j] {
                p.swap(i, j);
                break;
            }
        }

        p[i+1..].sort();
        p[i+1..].reverse();

        println!("{}", p.iter().join(" "));
        return;
    }
}
