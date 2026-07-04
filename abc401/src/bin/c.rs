use std::collections::VecDeque;

use proconio::*;

const M: usize = 1000_000_000;

fn main() {
    input! {n: usize, k: usize}

    let mut nt = VecDeque::from(vec![1; k]);
    let mut next = k;
    let mut sum = k;
    for _ in k..=n {
        nt.push_back(next);
        let f = nt.pop_front().unwrap();
        sum += next;
        sum += M - f;
        sum %= M;
        next = sum;
    }

    println!("{}", nt.pop_back().unwrap());
}
