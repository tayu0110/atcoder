#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: i64, mut x: i64, mut d: i64}

    if d == 0 {
        if x == 0 {
            println!("1");
        } else {
            println!("{}", n + 1);
        }
        return;
    }

    let mut map = std::collections::BTreeMap::new();
    for i in 0..=n {
        let l = i * x / d + (i-1) * i / 2;
        let r = i * x / d + (2*n-i-1) * i / 2;
        map.entry((i * x % d + d) % d).or_insert(vec![]).push((std::cmp::min(l, r), std::cmp::max(l, r)));
    }

    let mut res = 0;
    for (_, v) in map {
        let mut s = v;
        s.sort();
        let (mut min, mut max) = s[0];
        for (l, r) in s.into_iter().skip(1) {
            if l > max {
                res += max - min + 1;
                max = r;
                min = l;
            }
            max = std::cmp::max(max, r);
            min = std::cmp::min(min, l);
        }
        res += max - min + 1;
    }

    println!("{}", res);
}
