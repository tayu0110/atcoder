use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}
    a.sort();
    a.dedup();

    let mut set = HashSet::new();
    let mut nt = BinaryHeap::new();
    nt.push(Reverse(0));
    set.insert(0);

    for _ in 0..k {
        let Reverse(now) = nt.pop().unwrap();
        for &a in &a {
            if !set.contains(&(now + a)) {
                set.insert(now + a);
                nt.push(Reverse(now + a));
            }
        }
        set.remove(&now);
    }

    let Reverse(res) = nt.pop().unwrap();
    println!("{}", res)
}
