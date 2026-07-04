use proconio::*;

fn main() {
    input! {h: usize, w: usize, n: usize, a: [[usize; w]; h], b: [usize; n]}

    let mut ret = vec![vec![false; w]; h];
    for b in b {
        for i in 0..h {
            for j in 0..w {
                if a[i][j] == b {
                    ret[i][j] = true;
                }
            }
        }
    }

    println!(
        "{}",
        ret.into_iter()
            .map(|row| row.into_iter().filter(|&b| b).count())
            .max()
            .unwrap()
    )
}
