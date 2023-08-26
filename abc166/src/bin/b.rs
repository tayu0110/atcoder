use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    let mut set = (1..=n).collect::<HashSet<_>>();
    for _ in 0..k {
        input! {d: usize, a: [usize; d]}
        for a in a {
            set.remove(&a);
        }
    }
    println!("{}", set.len());
}
