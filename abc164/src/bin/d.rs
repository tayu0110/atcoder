#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {s: Chars}
    let n = s.len();

    let s = s.into_iter().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();

    let mut m = vec![0; n+1];
    let mut v = vec![0i64; 2019];
    v[0] = 1;
    let mut ten = 1;
    for i in (0..n).rev() {
        m[i] = (m[i+1] + s[i] * ten) % 2019;
        ten = ten * 10 % 2019;
        v[m[i] as usize] += 1;
    }

    let res = v.into_iter().map(|v| v * (v-1) / 2).sum::<i64>();
    println!("{}", res);
}
