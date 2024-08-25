use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {n: usize, a: [usize; n], m: usize, b: [usize; m], l: usize, c: [usize; l], q: usize, x: [usize; q]}

    let mut memo = FxHashSet::default();
    for a in a {
        for b in &b {
            for c in &c {
                memo.insert(a + b + c);
            }
        }
    }

    for x in x {
        if memo.contains(&x) {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
