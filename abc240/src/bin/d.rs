#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut stack = vec![];
    let mut res = 0;
    for a in a {
        if stack.is_empty() {
            stack.push((a, 1));
            res += 1;
        } else {
            match stack.last_mut() {
                Some((now, n)) if *now == a => {
                    *n += 1;
                    res += 1;
                    if *n == *now {
                        stack.pop().unwrap();
                        res -= a;
                    }
                },
                _ => {
                    stack.push((a, 1));
                    res += 1;
                }
            }

        }
        println!("{}", res);
    }

}
