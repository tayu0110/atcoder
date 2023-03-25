use itertools::Itertools;
use proconio::*;

fn main() {
    input! {h: usize, w: usize, a: [[u8; w]; h]}

    let mut b = vec![vec![' '; w]; h];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 0 {
                b[i][j] = '.';
            } else {
                b[i][j] = (a[i][j] - 1 + b'A') as char;
            }
        }
    }

    for i in 0..h {
        println!("{}", b[i].iter().join(""))
    }
}
