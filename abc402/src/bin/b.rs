use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {q: usize}

    let mut nt = VecDeque::new();
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {x: usize}
            nt.push_back(x);
        } else {
            println!("{}", nt.pop_front().unwrap())
        }
    }
}
