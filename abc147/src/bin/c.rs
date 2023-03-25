#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize}

    let mut t = vec![];
    for _ in 0..n {
        input! {k: usize, v: [(usize, usize); k]}

        t.push(v);
    }

    let mut res = 0;
    for i in 0..(1usize << n) {
        let mut bad = false;
        for j in 0..n {
            if i & (1 << j) != 0 {
                for &(x, y) in &t[j] {
                    if y == 0 {
                        if i & (1 << (x-1)) != 0 {
                            bad = true;
                        }
                    } else {
                        if i & (1 << (x-1)) == 0 {
                            bad = true;
                        }
                    }
                }
            }
        }

        if !bad {
            res = std::cmp::max(res, i.count_ones());
        }
    }

    println!("{}", res);
}
