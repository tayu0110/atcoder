#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, a: [usize; n]}

    let max = a.iter().max().unwrap();
    let pos = a.iter().position(|c| c == max).unwrap();
    let second = a
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != pos)
        .map(|(_, x)| *x)
        .max()
        .unwrap();

    for i in 0..n {
        if i == pos {
            println!("{}", second);
        } else {
            println!("{}", max);
        }
    }
}
