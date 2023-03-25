#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {x: Chars, n: usize, s: [Chars; n]}

    let mut map = std::collections::HashMap::new();
    for (i, &c) in x.iter().enumerate() {
        map.insert(c, (b'a' + i as u8) as char);
    }

    let mut t = s.iter().map(|s| s.iter().map(|c| *map.get(c).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    t.sort();

    for res in t {
        let s = res.into_iter().map(|c| x[(c as u8 - b'a') as usize]).collect::<String>();
        println!("{}", s);
    }
}
