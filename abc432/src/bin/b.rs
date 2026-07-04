use itertools::Itertools;
use proconio::*;

fn main() {
    input! {mut x: marker::Bytes}
    x.sort();
    if x[0] == b'0' {
        let pos = x.iter().position(|x| x != &b'0').unwrap();
        x.swap(0, pos);
    }
    println!("{}", x.iter().map(|b| b - b'0').join(""))
}
