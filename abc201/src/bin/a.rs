#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut a: [usize; 3]}
    a.sort();

    if a[2] - a[1] == a[1] - a[0] {
        println!("Yes");
    } else {
        println!("No");
    }
}
