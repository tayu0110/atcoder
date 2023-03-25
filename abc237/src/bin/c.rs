#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {mut s: Chars}

    let a = s.iter().take_while(|c| **c == 'a').count();
    if a == s.len() {
        println!("Yes");
        return;
    }

    let ra = s.iter().rev().take_while(|c| **c == 'a').count();
    while let Some(c) = s.pop() {
        if c != 'a' {
            s.push(c);
            break;
        }
    }
    s.reverse();
    while let Some(c) = s.pop() {
        if c != 'a' {
            s.push(c);
            break;
        }
    }

    let mut t = s.clone();
    t.reverse();
    if s != t {
        println!("No");
        return;
    }

    if a <= ra {
        println!("Yes");
    } else {
        println!("No");
    }
}
