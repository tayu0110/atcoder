#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, a: [usize; 3*n]}
    let mut a = a.into_iter().enumerate().map(|(i, v)| (v, i)).collect::<Vec<(_, _)>>();
    a.sort();
    let mut a = a.into_iter().skip(n).take(n).map(|(_, i)| i).collect::<Vec<_>>();
    a.sort();
    for res in a {
        println!("{}", res + 1);
    }
}
