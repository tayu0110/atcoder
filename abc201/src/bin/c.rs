#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {s: Chars}

    let mut res = 0;
    for i in 0..10000usize {
        let mut f = vec![false; 10];
        let t = format!("{:04}", i);
        for j in t.chars().map(|c| c.to_digit(10).unwrap()) {
            f[j as usize] = true;
        }

        let mut bad = false;
        for (&c, f) in s.iter().zip(f.into_iter()) {
            if c == 'o' && !f {
                bad = true;
            }
            if c == 'x' && f {
                bad = true;
            }
        }

        if !bad {
            res += 1;
        }
    }

    println!("{}", res);
}
