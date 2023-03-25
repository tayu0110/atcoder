#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, l: usize, a: [usize; n]}

    let mut nt = std::collections::BinaryHeap::new();
    nt.push(l);

    for a in a {
        if let Some(mut now) = nt.pop() {
            if a > now {
                println!("No");
                return;
            }

            now -= a;

            if now != 0 {
                nt.push(1);
    
                if now > 1 {
                    nt.push(now - 1);
                }
            }

        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
