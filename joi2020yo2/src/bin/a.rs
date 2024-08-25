use proconio::*;

fn rot(s: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = s.len();
    let mut new = vec![vec!['R'; n]; n];
    for i in 0..n {
        for j in 0..n {
            new[j][n - 1 - i] = s[i][j];
        }
    }
    new
}

fn main() {
    input! {n: usize, mut s: [marker::Chars; n], t: [marker::Chars; n]}

    let mut res = usize::MAX;
    for i in 0..4 {
        res = res.min(
            s.iter()
                .flatten()
                .zip(t.iter().flatten())
                .filter(|(s, t)| s != t)
                .count()
                + i.min(4 - i),
        );
        s = rot(s);
    }

    println!("{}", res);
}
