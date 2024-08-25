#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    const MOD: usize = 1_000_000_007;
    input! {s: Chars}
    let n = s.len();

    let mut memo = vec![1; n+1];
    for c in "chokudai".chars() {
        let mut tmp = vec![0; n+1];
        for i in 1..=n {
            if s[i-1] == c {
                tmp[i] = memo[i-1];
            }
        }

        for i in 0..n {
            tmp[i+1] += tmp[i];
            tmp[i+1] %= MOD;
        }

        std::mem::swap(&mut memo, &mut tmp);
    }

    println!("{}", memo[n]);
}
