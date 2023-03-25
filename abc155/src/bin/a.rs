#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut a: [usize; 3]}
    a.sort();

    if a[0] == a[1] && a[1] == a[2] {
        println!("No")
    } else if a[0] != a[1] && a[1] != a[2] && a[2] != a[0] {
        println!("No")
    } else {
        println!("Yes")
    }
}
