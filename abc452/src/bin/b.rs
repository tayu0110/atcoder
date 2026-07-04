use itertools::Itertools;
use proconio::*;

fn main() {
    input! {h: usize, w: usize}

    let mut ret = vec![vec!['.'; w]; h];
    for i in 0..h {
        ret[i][0] = '#';
        ret[i][w - 1] = '#';
    }
    for i in 0..w {
        ret[0][i] = '#';
        ret[h - 1][i] = '#';
    }
    for i in 0..h {
        println!("{}", ret[i].iter().join(""))
    }
}
