#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut map = std::collections::HashMap::new();
    for (i, &(x, y)) in p.iter().enumerate() {
        for &(nx, ny) in p.iter().skip(i+1) {
            if x == nx {
                *map.entry((std::cmp::min(y, ny), std::cmp::max(y, ny))).or_insert(std::collections::HashMap::new()).entry(x).or_insert(0) += 1;
            }
        }
    }

    let mut res = 0;
    for (_, v) in map {
        let mut sum = 0usize;
        for (_, v) in &v {
            sum += *v;
        }

        for (_, v) in v {
            sum -= v;
            res += v * sum;
        }
    }

    println!("{}", res);
}
