#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, s: Chars, t: Chars}

    if n == 1 {
        if s == t {
            println!("0")
        } else {
            println!("-1")
        }
        return;
    }

    let mut map = std::collections::HashMap::new();
    for &c in &t {
        *map.entry(c).or_insert(0) += 1;
    }

    {
        let mut m = std::collections::HashMap::new();
        for &c in &s {
            *m.entry(c).or_insert(0) += 1;
        }
        for (k, v) in m {
            if let Some(nv) = map.get(&k) {
                if nv != &v {
                    println!("-1");
                    return;
                }
            } else {
                println!("-1");
                return;
            }
        }
    }

    let (mut l, mut r) = (-1, n as i32);
    'base: while r - l > 1 {
        let m = ((r + l) / 2) as usize;

        let mut i = 0;
        let mut j = m;
        while i < n && j < n {
            while i < n && t[i] != s[j] {
                i += 1;
            }

            if i == n {
                l = m as i32;
                continue 'base;
            }

            j += 1;
            i += 1;
        }

        if j == n {
            r = m as i32;
        } else {
            l = m as i32;
        }
    }

    if r == n as i32 {
        println!("-1")
    } else {
        println!("{}", l + 1);
    }
}
