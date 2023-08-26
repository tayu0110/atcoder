use itertools::Itertools;
use proconio::input;

fn main() {
    input! {h: usize, w: usize, n: usize, p: [(usize, usize, usize, usize); n]}

    let mut res = vec![vec![0; w + 1]; h + 1];
    for (a, b, c, d) in p {
        res[c][d] += 1;
        res[a - 1][b - 1] += 1;
        res[c][b - 1] -= 1;
        res[a - 1][d] -= 1;
    }

    for i in 0..=h {
        for j in 0..w {
            res[i][j + 1] += res[i][j];
        }
    }
    for i in 0..h {
        for j in 0..=w {
            res[i + 1][j] += res[i][j];
        }
    }

    for i in 0..h {
        res[i].pop().unwrap();
        println!("{}", res[i].iter().join(" "))
    }
}
