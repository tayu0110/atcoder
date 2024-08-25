use proconio::*;
use std::collections::HashSet;

fn main() {
    input! {n: usize, s: [usize; n], q: usize}

    let s = s.into_iter().collect::<HashSet<_>>();
    for _ in 0..q {
        input! {t: [usize]}
        println!("{}", t.into_iter().filter(|t| !s.contains(t)).count() + n)
    }
}
