#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {s: Chars, mut k: usize}

    for c in s {
        let c = c.to_digit(10).unwrap() as usize;
        if c == 1 {
            if k == 1 {
                println!("1");
                return;
            }
            k -= 1;
        } else {
            println!("{}", c);
            return;
        }
    }
}
