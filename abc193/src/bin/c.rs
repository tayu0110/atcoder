#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}

    let mut set = std::collections::HashSet::new();
    for i in (2..=n).take_while(|i| *i * *i <= n) {
        let mut now = i;
        while now < n {
            now *= i;
            if now <= n {
                set.insert(now);
            }
        }
    }

    println!("{}", n - set.len());
}
