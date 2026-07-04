use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {_: usize, q: usize, a: [usize; q]}

    let mut set = BTreeSet::new();
    let mut cnt = 0;
    for a in a {
        if set.contains(&a) {
            if set.contains(&(a - 1)) && set.contains(&(a + 1)) {
                cnt += 1;
            } else if !set.contains(&(a - 1)) && !set.contains(&(a + 1)) {
                cnt -= 1;
            }
            set.remove(&a);
        } else {
            if set.contains(&(a - 1)) && set.contains(&(a + 1)) {
                cnt -= 1;
            } else if !set.contains(&(a - 1)) && !set.contains(&(a + 1)) {
                cnt += 1;
            }
            set.insert(a);
        }
        println!("{cnt}")
    }
}
