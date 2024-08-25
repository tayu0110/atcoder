#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {h: usize, w: usize, a: [[i32; w]; h], b: [[i32; w]; h]}

    let c = {
        let mut buf = vec![vec![0; w]; h];
        for i in 0..h {
            for j in 0..w {
                buf[i][j] = (a[i][j] - b[i][j]).abs();
            }
        }
        buf
    };

    const MAX: usize = 6500;
    let mut memo = vec![vec![vec![false; MAX]; w]; h];
    memo[0][0][c[0][0] as usize] = true;

    for i in 0..h {
        for j in 0..w {
            for k in 0..MAX {
                if !memo[i][j][k] {
                    continue;
                }
                if i + 1 < h {
                    if k as i32 + c[i+1][j] < MAX as i32 {
                        memo[i+1][j][k + c[i+1][j] as usize] = true;
                    }
                    memo[i+1][j][(k as i32 - c[i+1][j]).unsigned_abs() as usize] = true;
                }
                if j + 1 < w {
                    if k as i32 + c[i][j+1] < MAX as i32 {
                        memo[i][j+1][k + c[i][j+1] as usize] = true;
                    }
                    memo[i][j+1][(k as i32 - c[i][j+1]).unsigned_abs() as usize] = true;
                }
            }
        }
    }

    let mut res = std::usize::MAX;
    for i in 0..MAX {
        if memo[h-1][w-1][i] {
            res = i;
            break;
        }
    }

    println!("{}", res);
}
