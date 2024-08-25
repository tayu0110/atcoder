use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    let mut cnt = 0;
    for a in a.iter_mut() {
        while *a % 2 == 0 {
            *a /= 2;
            cnt += 1;
        }
    }

    let mut nt = a.into_iter().map(Reverse).collect::<BinaryHeap<_>>();
    for _ in 0..cnt {
        let Reverse(a) = nt.pop().unwrap();
        nt.push(Reverse(a * 3));
    }

    let Reverse(res) = nt.pop().unwrap();
    println!("{}", res)
}
