use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, h: [usize; n], b: [usize; m]}

    let mut t = 0;
    let mut set = b
        .iter()
        .copied()
        .enumerate()
        .map(|(i, b)| (b, i))
        .collect::<BTreeSet<_>>();
    for &h in &h {
        if let Some(&(b, i)) = set.range((h, 0)..).next() {
            t += 1;
            set.remove(&(b, i));
        }
    }

    if t >= k {
        println!("Yes");
        return;
    }

    let mut t = 0;
    let mut set = h
        .iter()
        .copied()
        .enumerate()
        .map(|(i, h)| (h, i))
        .collect::<BTreeSet<_>>();
    for b in b {
        if let Some(&(h, i)) = set.range(..=(b, usize::MAX)).next_back() {
            t += 1;
            set.remove(&(h, i));
        }
    }

    if t >= k {
        println!("Yes")
    } else {
        println!("No")
    }
}
