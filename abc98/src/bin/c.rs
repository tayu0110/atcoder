#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {_n: usize, s: Chars}

    let mut now = s.iter().skip(1).filter(|&&c| c == 'E').count();
    let mut res = now;

    for v in s.windows(2) {
        if v[1] == 'E' {
            now -= 1;
        }
        if v[0] == 'W' {
            now += 1;
        }

        res = std::cmp::min(res, now);
    }

    println!("{}", res);
}
