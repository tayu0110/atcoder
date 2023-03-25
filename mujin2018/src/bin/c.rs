#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn solve(n: usize, m: usize, s: &Vec<Vec<char>>) -> usize {
    let mut v = vec![vec![0; m + 2]; n + 2];
    for i in 2..=n {
        for j in 1..=m {
            if s[i][j] == '.' && s[i - 1][j] == '.' {
                v[i][j] = v[i - 1][j] + 1;
            }
        }
    }

    let mut u = vec![vec![0; m + 2]; n + 2];
    for i in 1..=n {
        for j in 2..=m {
            if s[i][j] == '.' && s[i][j - 1] == '.' {
                u[i][j] = v[i][j - 1] + u[i][j - 1];
            }
        }
    }

    u.iter().flatten().sum()
}

fn transpose(s: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (n, m) = (s.len(), s[0].len());

    let mut res = vec![vec!['.'; n]; m];
    for i in 0..n {
        for j in 0..m {
            res[m - j - 1][i] = s[i][j];
        }
    }

    res
}

fn main() {
    input! {mut n: usize, mut m: usize, s: [Chars; n]}

    let mut s = {
        let mut ns = vec![vec!['#'; m + 2]; n + 2];
        for i in 0..n {
            ns[i + 1][1..=m].copy_from_slice(&s[i]);
        }
        ns
    };

    let mut res = 0;
    for _ in 0..4 {
        res += solve(n, m, &s);
        s = transpose(&s);
        std::mem::swap(&mut n, &mut m);
    }

    println!("{}", res);
}
