#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, a: [usize; n]}
    let set = a.into_iter().collect::<std::collections::HashSet<_>>();

    for i in 0..=n {
        if !set.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}
