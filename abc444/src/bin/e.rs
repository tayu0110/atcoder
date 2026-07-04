use std::collections::{BTreeMap, btree_map::Entry};

use proconio::*;

fn main() {
    input! {n: usize, d: i32, a: [i32; n]};

    let mut set = BTreeMap::<i32, i32>::new();
    let (mut l, mut r) = (0, 0);
    let mut ret = 0usize;
    while l < n {
        while r < n && set.range(a[r] - d + 1..a[r] + d).next().is_none() {
            *set.entry(a[r]).or_default() += 1;
            r += 1;
        }

        ret += r - l;
        let entry = set.entry(a[l]).and_modify(|v| *v -= 1);
        match entry {
            Entry::Occupied(entry) => {
                if *entry.get() == 0 {
                    entry.remove();
                }
            }
            _ => unreachable!(),
        }
        l += 1;
    }

    println!("{}", ret);
}
