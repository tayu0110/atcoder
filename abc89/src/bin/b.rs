#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, s: [char; n]}

    let set = s.into_iter().collect::<std::collections::HashSet<_>>();

    if set.len() == 3 {
        println!("Three")
    } else {
        println!("Four")
    }
}
