#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let mut set = std::collections::HashSet::new();
    for v in a {
        for i in (1..=v).take_while(|i| *i * *i <= v) {
            if v % i == 0 {
                set.insert(i);
                set.insert(v / i);
            }
        }
    }
    set.remove(&1);

    let mut d = vec![std::usize::MAX; m+1];
    for v in set {
        if v > m {
            continue;
        }
        if d[v] == std::usize::MAX {
            for i in (1..=m).take_while(|i| *i * v <= m) {
                d[i * v] = v;
            }
        }
    }

    println!("{}", d.iter().skip(1).filter(|v| **v == std::usize::MAX).count());
    for (res, _) in d.into_iter().enumerate().skip(1).filter(|&(_, v)| v == std::usize::MAX) {
        println!("{}", res);
    }
}
