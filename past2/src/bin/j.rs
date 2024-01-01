use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: String}

    let mut buf = vec![];
    for c in s.chars() {
        if c == ')' {
            let mut keep = vec![];
            while let Some(p) = buf.pop() {
                if p == '(' {
                    break;
                }

                keep.push(p);
            }

            buf.extend(keep.iter().cloned().rev().chain(keep.iter().cloned()));
        } else {
            buf.push(c);
        }
    }

    println!("{}", buf.iter().join(""))
}
