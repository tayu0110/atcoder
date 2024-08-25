use itertools::Itertools;
use proconio::*;

fn main() {
    input! {r: usize, c: usize, mut b: [marker::Chars; r]}

    for i in 0..r {
        for j in 0..c {
            let nc = b[i][j];
            if nc.is_ascii_digit() {
                let nc = nc as usize - b'0' as usize;
                for ni in 0..r {
                    for nj in 0..c {
                        if ni.max(i) - ni.min(i) + nj.max(j) - nj.min(j) <= nc && b[ni][nj] == '#' {
                            b[ni][nj] = '.';
                        }
                    }
                }
                b[i][j] = '.'
            }
        }
    }

    for s in b {
        println!("{}", s.iter().join(""))
    }
}
