#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn check(n: usize, s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> bool {
    let udlr = |v: &Vec<Vec<char>>| {
        let (mut u, mut d, mut l, mut r) = (n, 0, n, 0);
        for i in 0..n {
            for j in 0..n {
                if v[i][j] == '#' {
                    u = std::cmp::min(u, i);
                    d = std::cmp::max(d, i);
                    l = std::cmp::min(l, j);
                    r = std::cmp::max(r, j);
                }
            }
        }
        (u, d, l, r)
    };
    let (su, sd, sl, sr) = udlr(s);
    let (tu, td, tl, tr) = udlr(t);

    if sd - su != td - tu || sr - sl != tr - tl {
        return false;
    }

    for (sh, th) in (su..=sd).zip(tu..=td) {
        for (sw, tw) in (sl..=sr).zip(tl..=tr) {
            if s[sh][sw] != t[th][tw] {
                return false;
            }
        }
    }

    true
}

fn main() {
    input! {n: usize, mut s: [Chars; n], t: [Chars; n]}

    let rotate = |v: &mut Vec<Vec<char>>| {
        let mut buf = vec![vec!['0'; n]; n];

        for i in 0..n {
            for j in 0..n {
                buf[j][n-1-i] = v[i][j];
            }
        }

        *v = buf;
    };

    for _ in 0..4 {
        if check(n, &s, &t) {
            println!("Yes");
            return;
        }

        rotate(&mut s);
    }

    println!("No");
}
