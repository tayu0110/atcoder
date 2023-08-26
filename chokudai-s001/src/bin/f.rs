use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0;
    let mut set = BTreeSet::new();
    for a in a {
        if set.range(a..).next().is_none() {
            res += 1;
        }
        set.insert(a);
    }

    println!("{}", res)
}
