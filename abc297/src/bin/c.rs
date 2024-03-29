use itertools::Itertools;
use proconio::*;

fn main() {
    input! {h: usize, w: usize, mut s: [marker::Chars; h]}

    for i in 0..h {
        for j in 0..w - 1 {
            if s[i][j] == 'T' && s[i][j + 1] == 'T' {
                s[i][j] = 'P';
                s[i][j + 1] = 'C';
            }
        }
        println!("{}", s[i].iter().join(""))
    }
}
