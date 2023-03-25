#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {s: Chars, k: usize}
    let len = s.len();

    let (mut l, mut r) = (0, 0);
    let mut dot = 0;

    let mut res = 0;
    while l < len {
        while r < len && dot <= k {
            if s[r] == '.' {
                dot += 1;
            }
            r += 1;
        }

        if dot <= k {
            res = std::cmp::max(res, r - l);
        } else if dot > k && r > l {
            res = std::cmp::max(res, r - l - 1);
        }

        if l < r && s[l] == '.' {
            dot -= 1;
        }
        l += 1;
        
        if r < l {
            r += 1;
        }
    }

    println!("{}", res);
}
