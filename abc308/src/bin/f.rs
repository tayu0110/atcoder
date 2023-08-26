use std::collections::BinaryHeap;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut p: [usize; n], l: [usize; m], d: [usize; m]}

    let mut res = p.iter().sum::<usize>();

    let mut ld = l.into_iter().zip(d).collect::<Vec<_>>();
    ld.sort();
    ld.reverse();
    // eprintln!("{ld:?}");

    p.sort();
    p.reverse();
    let mut nt = BinaryHeap::new();

    while let Some(now) = p.pop() {
        while let Some((l, d)) = ld.pop() {
            if l > now {
                ld.push((l, d));
                break;
            }

            nt.push(d);
        }

        if let Some(d) = nt.pop() {
            res -= d;
        }
    }

    println!("{}", res)
}
