use proconio::*;
use std::collections::VecDeque;

fn main() {
    input! {q: usize}

    let mut nt = VecDeque::new();
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {x: String}
            nt.push_back(x);
        } else if ty == 2 {
            println!("{}", nt.front().unwrap());
        } else {
            nt.pop_front();
        }
    }
}
