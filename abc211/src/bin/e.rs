#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn check(width: usize, v: [usize; 3]) -> bool {
    for i in 0..width {
        if v[1] & (1 << i) != 0 {
            if v[0] & (1 << i) != 0 {
                continue;
            }
            if v[2] & (1 << i) != 0 {
                continue;
            }
            if i > 0 && v[1] & (1 << (i-1)) != 0 {
                continue;
            }
            if i+1 < width && v[1] & (1 << (i+1)) != 0 {
                continue;
            }
            return false;
        }
    }

    eprintln!("{:08b}", v[0]);
    eprintln!("{:08b}", v[1]);
    eprintln!("{:08b}\n", v[2]);

    true
}

fn solve(n: usize, rem: usize, v: [usize; 3], t: &Vec<Vec<Vec<usize>>>) -> usize {
    if n == t.len() {
        if rem == 0 && check(t.len(), [v[1], v[2], 0]) {
            return 1;
        } else {
            return 0;
        }
    }

    let mut res = 0;
    for i in 0..std::cmp::min(rem+1, t[n].len()) {
        for j in 0..t[n][i].len() {
            if check(t.len(), [v[1], v[2], t[n][i][j]]) {
                res += solve(n+1, rem - i, [v[1], v[2], t[n][i][j]], t);
            }
        }
    }

    res
}

fn main() {
    input! {n: usize, k: usize, s: [Chars; n]}

    let mut t = vec![vec![vec![]; n+1]; n];
    for (i, s) in s.iter().enumerate() {
        let mut k = 0;
        for (i, c) in s.iter().enumerate() {
            if c == &'#' {
                k |= 1 << i;
            }
        }

        let mut l: usize = ((1 << n) - 1) ^ k;

        while l > 0 {
            t[i][l.count_ones() as usize].push(l | k);
            l = (l-1) & l;
        }
        t[i][0].push(k);
    }

    let mut res = 0;
    for i in 0..std::cmp::min(k+1, t[0].len()) {
        for j in 0..t[0][i].len() {
            res += solve(1, k-i, [0, 0, t[0][i][j]], &t);
        }
    }

    println!("{}", res);
}
