use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {l: usize, p: [(usize, usize)]}
    let mut set = BTreeSet::new();
    set.insert((0, l));

    for (c, x) in p {
        if let Some(&(p, n)) = set.range(..(x, 0)).next_back() {
            if c == 1 {
                set.remove(&(p, n));
                set.insert((p, x));
                set.insert((x, n));
            } else {
                println!("{}", n - p)
            }
        }
    }
}
