use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; n], b: [usize; n], c: [usize; n], d: [usize; n]}

    let mut set = HashSet::new();
    for a in a {
        for &b in &b {
            set.insert(a + b);
        }
    }

    for c in c {
        for &d in &d {
            if c + d > k {
                continue;
            }

            if set.contains(&(k - c - d)) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No")
}
