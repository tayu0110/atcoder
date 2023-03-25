use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut buf = VecDeque::new();
    for i in 0..n {
        if i % 2 == 0 {
            buf.push_back(a[i]);
        } else {
            buf.push_front(a[i]);
        }
    }

    if n % 2 == 0 {
        println!("{}", buf.into_iter().join(" "))
    } else {
        println!("{}", buf.into_iter().rev().join(" "))
    }
}
