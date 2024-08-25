#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {x: i32, y: i32}

    if y < 0 {
        println!("-1");
        return;
    }

    if x != 0 && y == 0 {
        println!("-1");
        return;
    }

    if x % 2 == 0 {
        if y % 4 == 0 {
            let res = y / 2;
            if res < x.abs() {
                println!("-1");
            } else {
                println!("{}", res);
            }
        } else {
            println!("-1");
        }
    } else if y % 4 == 2 {
        let res = y / 2;
        if res < x.abs() {
            println!("-1");
        } else {
            println!("{}", res);
        }
    } else {
        println!("-1");
    }
}
