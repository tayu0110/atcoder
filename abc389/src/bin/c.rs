use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {q: usize}

    let mut nt = VecDeque::new();
    let mut sum = 0;
    let mut removed = 0;
    for _ in 0..q {
        input! {ty: u8}
        if ty == 1 {
            input! {l: usize}
            nt.push_back((sum, l));
            sum += l;
        } else if ty == 2 {
            let (_, l) = nt.pop_front().unwrap();
            removed += l;
        } else {
            input! {k: usize}
            let (pos, _) = nt[k - 1];
            println!("{}", pos - removed);
        }
    }
}
