#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {_n: usize, q: usize, s: Chars, p: [(usize, usize); q]}

    let mut t = vec![];
    let mut prev = '\0';
    for (i, c) in s.into_iter().enumerate() {
        if c != prev {
            t.push((i, c));
            prev = c;
        }
    }

    if t.len() == 1 {
        for _ in p {
            println!("0");
        }
        return;
    }

    let get_index = |i: usize| {
        let (mut l, mut r) = (-1 as i32, t.len() as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            let (j, _) = t[m as usize];
            if i <= j {
                r = m;
            } else {
                l = m;
            }
        }
        if r == t.len() as i32 {
            return l as usize;
        }
        if t[r as usize].0 > i {
            l as usize
        } else {
            r as usize
        }
    };

    for (l, r) in p {
        let (l, r) = (get_index(l - 1), get_index(r - 1));
        let (_, c) = t[l];
        let (_, d) = t[r];
        let diff = if c == d { r - l } else { r + 1 - l };
        println!("{}", (diff + 1) / 2);
    }
}
