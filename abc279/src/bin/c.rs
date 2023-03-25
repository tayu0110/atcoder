#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {h: usize, w: usize, s: [Chars; h], t: [Chars; h]}

    let mut map = std::collections::HashMap::new();
    for i in 0..w {
        let mut ns = String::new();
        for j in 0..h {
            ns.push(s[j][i]);
        }

        *map.entry(ns).or_insert(0) += 1;
    }

    for i in 0..w {
        let mut nt = String::new();
        for j in 0..h {
            nt.push(t[j][i]);
        }

        if !map.contains_key(&nt) {
            println!("No");
            return;
        }

        *map.entry(nt.clone()).or_insert(0) -= 1;
        if *map.get(&nt).unwrap() == 0 {
            map.remove(&nt);
        }
    }

    println!("Yes");
}
