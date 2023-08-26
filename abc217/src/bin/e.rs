use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {q: usize}
    let mut nt = BinaryHeap::new();
    let mut buf = vec![];
    let mut res = vec![];

    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: usize}

            buf.push(x);
        } else if t == 2 {
            if let Some(Reverse(x)) = nt.pop() {
                res.push(x);
            } else {
                res.push(buf.drain(..1).next().unwrap());
            }
        } else {
            for x in buf.drain(..) {
                nt.push(Reverse(x));
            }
        }
    }

    for res in res {
        println!("{}", res)
    }
}
