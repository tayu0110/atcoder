#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn transpose(s: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (h, w) = (s.len(), s[0].len());
    let mut t = vec![vec!['.'; h]; w];
    for i in 0..h {
        for j in 0..w {
            t[j][h - i - 1] = s[i][j];
        }
    }

    t
}

fn f(s: &[Vec<char>]) -> (usize, usize) {
    let (mut r, mut c) = (std::usize::MAX, std::usize::MAX);
    for (i, v) in s.iter().enumerate() {
        for (j, w) in v.iter().enumerate() {
            if w == &'#' {
                r = std::cmp::min(i, r);
                c = std::cmp::min(j, c);
            }
        }
    }

    (r, c)
}

fn check(s: &[Vec<char>], t: &[Vec<char>]) -> Option<usize> {
    let (rs, cs) = f(s);
    let (rt, ct) = f(t);
    let mut res = 0;

    for (is, it) in (rs..s.len()).zip(rt..t.len()) {
        for (js, jt) in (cs..s[0].len()).zip(ct..t[0].len()) {
            if s[is][js] != t[it][jt] {
                return None;
            }

            if s[is][js] == '#' {
                res += 1;
            }
        }
    }

    Some(res)
}

fn main() {
    input! {n: usize, mut s: [Chars; n], t: [Chars; n]}
    let cnt = s.iter().flatten().filter(|c| **c == '#').count();
    if cnt != t.iter().flatten().filter(|c| **c == '#').count() {
        println!("No");
        return;
    }

    for _ in 0..4 {
        if let Some(res) = check(&s, &t) {
            if res == cnt {
                println!("Yes");
                return;
            }
        }
        s = transpose(s);
    }

    println!("No");
}
