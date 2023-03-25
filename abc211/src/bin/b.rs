#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {s: [String; 4]}
    let set = s.into_iter().collect::<std::collections::HashSet<_>>();
    if set.len() == 4 {
        println!("Yes");
    } else {
        println!("No");
    }
}
