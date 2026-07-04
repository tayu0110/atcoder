use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {q: usize}

    let mut res = vec![];
    let mut now = 0;
    let mut nt = VecDeque::new();
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            nt.push_back(now);
        } else if ty == 2 {
            input! {t: usize}
            now += t;
        } else {
            input! {h: usize}
            let mut r = 0;
            while !nt.is_empty() && nt[0] + h <= now {
                nt.pop_front();
                r += 1;
            }
            res.push(r);
        }
    }
    println!("{}", res.iter().join("\n"))
}
