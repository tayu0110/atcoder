use std::cmp::Reverse;

use proconio::*;

fn main() {
    input! {q: usize}

    let mut buf = vec![];
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {x: usize}
            buf.push(x);
        } else {
            buf.sort_unstable_by_key(|&b| Reverse(b));
            println!("{}", buf.pop().unwrap());
        }
    }
}
