#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, s: [String; n]}

    let mut map = std::collections::HashMap::new();
    for s in s {
        let c = s.chars().collect::<Vec<_>>();
        *map.entry(c[0]).or_insert(0) += 1;
    }

    let mut res = 0usize;
    for (i, c) in "MARCH".chars().enumerate() {
        for (j, d) in "MARCH".chars().enumerate().skip(i + 1) {
            for e in "MARCH".chars().skip(j + 1) {
                if c == d || d == e || e == c {
                    continue;
                }
                let (c, d, e) = (
                    *map.get(&c).unwrap_or(&0),
                    *map.get(&d).unwrap_or(&0),
                    *map.get(&e).unwrap_or(&0),
                );
                res += c * d * e;
            }
        }
    }

    println!("{}", res)
}
