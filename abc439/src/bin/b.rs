use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {mut n: u32}

    let mut set = FxHashSet::default();
    while n != 1 && !set.contains(&n) {
        set.insert(n);
        let mut new = 0;
        for c in n.to_string().bytes() {
            new += (c - b'0') as u32 * (c - b'0') as u32;
        }
        n = new;
    }

    if n == 1 {
        println!("Yes")
    } else {
        println!("No")
    }
}
