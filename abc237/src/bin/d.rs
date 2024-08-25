#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, s: Chars}

    let mut prev = vec![std::usize::MAX; n+1];
    let mut next = vec![std::usize::MAX; n+1];

    for (i, c) in s.into_iter().enumerate() {
        if c == 'L' {
            if prev[i] == std::usize::MAX {
                prev[i] = i+1;
                next[i+1] = i;
            } else {
                let p = prev[i];
                prev[i+1] = p;
                prev[i] = i+1;
                next[p] = i+1;
                next[i+1] = i;
            }
        } else if next[i] == std::usize::MAX {
            next[i] = i+1;
            prev[i+1] = i;
        } else {
            let n = next[i];
            next[i] = i+1;
            next[i+1] = n;
            prev[n] = i+1;
            prev[i+1] = i;
        }
    }

    for i in 0..n+1 {
        if prev[i] == std::usize::MAX {
            print!("{}", i);
            let mut now = i;
            while next[now] != std::usize::MAX {
                print!(" {}", next[now]);
                now = next[now];
            }
            println!("");
            return;
        }
    }
}
