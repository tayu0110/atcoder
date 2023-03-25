#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, s: [String; n]}

    let mut set = std::collections::HashSet::new();
    for s in s {
        if set.contains(&s) {
            println!("No");
            return;
        }

        set.insert(s.clone());

        let s = s.chars().collect::<Vec<_>>();
        if s[0] != 'H' && s[0] != 'D' && s[0] != 'C' && s[0] != 'S' {
            println!("No");
            return;
        }

        if "A23456789TJQK".chars().all(|c| c != s[1]) {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
