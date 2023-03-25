#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

const MOD: usize = 998244353;

fn main() {
    input! {_n: usize, mut s: Chars, mut t: Chars}

    for (c, d) in s.iter_mut().zip(t.iter_mut()) {
        let (a, b) = (std::cmp::min(*c, *d), std::cmp::max(*c, *d));
        *c = a;
        *d = b;
    }

    let mut ns = 0;
    for c in s {
        let c = (c as u8 - b'0') as usize;
        ns = (ns * 10 + c) % MOD;
    }
    let mut nt = 0;
    for c in t {
        let c = (c as u8 - b'0') as usize;
        nt = (nt * 10 + c) % MOD;
    }

    println!("{}", (ns * nt) % MOD);
}
