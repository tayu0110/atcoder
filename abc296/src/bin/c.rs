use proconio::*;
use std::collections::HashSet;

fn main() {
    input! {n: usize, x: i64, a: [i64; n]}
    let set = a.iter().cloned().collect::<HashSet<_>>();
    if a.iter().any(|a| set.contains(&(a + x))) {
        println!("Yes")
    } else {
        println!("No")
    }
}
