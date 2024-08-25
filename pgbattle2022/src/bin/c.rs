#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, a: [usize; n]}

    let mut stack = vec![];
    for a in a {
        if stack.is_empty() {
            stack.push((a, a == 1));
        } else if stack.last().unwrap().1 {
            if a == stack.last().unwrap().0 + 1 {
                stack.push((a, true));
            } else if a == 1 {
                stack.push((a, true));
            } else {
                let mut prev = 0;
                while let Some((val, f)) = stack.pop() {
                    if !f || (val + 1 == a && prev == 1) {
                        stack.push((val, f));
                        break;
                    }
                    prev = val;
                }
                if stack.is_empty() {
                    stack.push((a, a == 1));
                } else {
                    stack.push((a, stack.last().unwrap().1 && stack.last().unwrap().0 + 1 == a));
                }
            }
        } else {
            stack.push((a, a == 1));
        }
    }

    while let Some((val, f)) = stack.pop() {
        if !f {
            stack.push((val, f));
            break;
        }
    }

    println!("{}", stack.iter().map(|(val, _)| val).sum::<usize>());
}
