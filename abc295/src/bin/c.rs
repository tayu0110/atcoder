use std::collections::BinaryHeap;

use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut a = a.into_iter().collect::<BinaryHeap<_>>();
    let mut res = 0;
    while let Some(now) = a.pop() {
        if let Some(next) = a.pop() {
            if now == next {
                res += 1;
            } else {
                a.push(next);
            }
        }
    }

    println!("{}", res)
}
