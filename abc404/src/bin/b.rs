use proconio::*;

fn rotate(s: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = s.len();
    let mut new = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            new[j][n - 1 - i] = s[i][j];
        }
    }
    new
}

fn main() {
    input! {n: usize, mut s: [marker::Bytes; n], t: [marker::Bytes; n]}

    let mut res = usize::MAX;
    for i in 0..4 {
        res = res.min(
            s.iter()
                .flatten()
                .zip(t.iter().flatten())
                .filter(|(s, t)| s != t)
                .count()
                + i,
        );
        s = rotate(s);
    }

    println!("{res}")
}
