#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn rec(len: usize, now: &mut String, set: &mut std::collections::HashSet<usize>) {
    if !now.is_empty() {
        set.insert(now.parse().unwrap());
    }

    if now.len() == len {
        return;
    }

    for c in "357".chars() {
        now.push(c);
        rec(len, now, set);
        now.pop().unwrap();
    }
}

#[fastout]
fn main() {
    input! {n: Chars}

    let mut set = std::collections::HashSet::new();
    rec(n.len(), &mut "".to_string(), &mut set);

    let n = n.into_iter().fold(0usize, |s, v| s * 10 + v.to_digit(10).unwrap() as usize);
    println!("{}", set.into_iter().filter(|c| *c <= n && c.to_string().chars().any(|c| c == '7') && c.to_string().chars().any(|c| c == '5') && c.to_string().chars().any(|c| c == '3')).count());
}
