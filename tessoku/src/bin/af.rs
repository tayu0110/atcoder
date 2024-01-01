use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize}

    let mut mex = vec![usize::MAX; n + 1];
    for i in 0..=n {
        let mut set = (0..3).collect::<BTreeSet<_>>();
        if i >= a {
            set.remove(&mex[i - a]);
        }
        if i >= b {
            set.remove(&mex[i - b]);
        }
        mex[i] = *set.first().unwrap();
    }

    if mex[n] == 0 {
        println!("Second")
    } else {
        println!("First")
    }
}
