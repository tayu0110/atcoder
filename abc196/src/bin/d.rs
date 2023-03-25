#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn rec(now: usize, a: usize, b: usize, t: &mut Vec<Vec<u8>>) -> usize {
    let (h, w) = (t.len(), t[0].len());

    if now == h*w && a == 0 && b == 0 {
        return 1;
    }
    
    let (r, c) = (now / w, now % w);

    if t[r][c] == 1 {
        return rec(now+1, a, b, t);
    }

    let mut res = 0;

    if a > 0 && c+1 < w && t[r][c+1] == 0 {
        t[r][c] = 1;
        t[r][c+1] = 1;
        res += rec(now+2, a-1, b, t);
        t[r][c] = 0;
        t[r][c+1] = 0;
    }

    if a > 0 && r+1 < h && t[r+1][c] == 0 {
        t[r][c] = 1;
        t[r+1][c] = 1;
        res += rec(now+1, a-1, b, t);
        t[r][c] = 0;
        t[r+1][c] = 0;
    }

    if b > 0 {
        t[r][c] = 1;
        res += rec(now+1, a, b-1, t);
        t[r][c] = 0;
    }

    res
}

#[fastout]
fn main() {
    input! {h: usize, w: usize, a: usize, b: usize}

    let mut t = vec![vec![0; w]; h];
    println!("{}", rec(0, a, b, &mut t));
}
