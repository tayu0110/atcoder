use std::collections::HashSet;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {_: usize, mut r: i64, mut c: i64, s: marker::Bytes}

    let (mut sr, mut sc) = (0, 0);
    let mut memo = HashSet::new();
    memo.insert((0, 0));
    let mut res = vec![];
    for s in s {
        match s {
            b'N' => {
                sr += 1;
                r += 1;
            }
            b'W' => {
                sc += 1;
                c += 1;
            }
            b'S' => {
                sr -= 1;
                r -= 1;
            }
            b'E' => {
                sc -= 1;
                c -= 1;
            }
            _ => {}
        }
        memo.insert((sr, sc));
        res.push(memo.contains(&(r, c)) as u8);
    }

    println!("{}", res.iter().join(""))
}
