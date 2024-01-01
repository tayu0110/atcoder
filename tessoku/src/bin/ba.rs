use proconio::*;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {q: usize}

    let mut nt = BinaryHeap::new();
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {x: usize}
            nt.push(Reverse(x));
        } else if ty == 2 {
            let Reverse(res) = nt.peek().unwrap();
            println!("{res}");
        } else {
            nt.pop();
        }
    }
}
