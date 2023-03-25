#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}

    for x in n.. {
        if x % 111 == 0 {
            println!("{}", x);
            return;
        }
    }
}
