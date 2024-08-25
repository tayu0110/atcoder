use std::cmp::Reverse;

use proconio::*;

fn main() {
    input! {n: usize, p: [(String, usize); n]}

    let mut p = p.into_iter().enumerate().collect::<Vec<_>>();

    p.sort_unstable_by_key(|(_, (s, p))| (s.clone(), Reverse(*p)));

    for (i, _) in p {
        println!("{}", i + 1);
    }
}
