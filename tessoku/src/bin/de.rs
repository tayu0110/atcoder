use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; k]}

    let mut grundy = vec![0; n + 1];
    for i in 0..=n {
        let mut set = BTreeSet::new();
        for &a in &a {
            if i < a {
                continue;
            }

            set.insert(grundy[i - a]);
        }

        let mut mex = 0;
        for s in set {
            if s != mex {
                break;
            }
            mex += 1;
        }

        grundy[i] = mex;
    }

    if grundy[n] > 0 {
        println!("First")
    } else {
        println!("Second")
    }
}
