use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {mut s: usize}

    let mut set = HashSet::new();
    for i in 1.. {
        if set.contains(&s) {
            println!("{}", i);
            return;
        }

        set.insert(s);
        if s % 2 == 0 {
            s /= 2;
        } else {
            s = 3 * s + 1;
        }
    }
}
