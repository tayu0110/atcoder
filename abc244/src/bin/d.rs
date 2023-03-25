#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {s: [char; 3], t: [char; 3]}

    {
        let mut s = s.clone();
        let mut t = t.clone();
        s.sort();
        t.sort();
        if s != t {
            println!("No");
            return;
        }
    }

    if t == s || t == vec![s[2], s[0], s[1]] || t == vec![s[1], s[2], s[0]] {
        println!("Yes");
    } else {
        println!("No");
    }
}
