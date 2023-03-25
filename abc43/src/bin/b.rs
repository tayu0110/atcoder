#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {s: Chars}

    let mut stack = vec![];
    for c in s {
        if c == 'B' {
            if !stack.is_empty() {
                stack.pop().unwrap();
            }
        } else {
            stack.push(c);
        }
    }

    println!("{}", stack.iter().join(""))
}
