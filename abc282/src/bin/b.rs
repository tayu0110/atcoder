#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, s: [Chars; n]}

    let mut t = vec![];
    for s in s {
        let mut k = 0;
        for i in 0..m {
            if s[i] == 'o' {
                k |= 1 << i;
            }
        }

        t.push(k);
    }

    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            if t[i] | t[j] == (1 << m) - 1 {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
