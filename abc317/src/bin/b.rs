use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let min = *a.iter().min().unwrap();
    let set = a.into_iter().collect::<HashSet<_>>();
    for now in min.. {
        if !set.contains(&now) {
            println!("{now}");
            return;
        }
    }
}
