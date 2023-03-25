#[allow(unused_imports)]
use proconio::{input, marker::Bytes, source::line::LineSource};

fn main() {
    input! {s: Bytes, t: Bytes};

    let mut w = vec![vec![]; 26];
    for (i, v) in s.iter().enumerate() {
        w[(*v - b'a') as usize].push(i);
    }

    let mut res = 0;
    let mut nt = 0;
    for v in t {
        let vi = (v - b'a') as usize;
        if w[vi].is_empty() {
            println!("-1");
            std::process::exit(0);
        }

        let (mut l, mut r) = (-1, w[vi].len() as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if w[vi][m as usize] >= nt {
                r = m;
            } else {
                l = m;
            }
        }

        if r == w[vi].len() as i32 {
            res += s.len() - nt;
            res += w[vi][0] + 1;
            nt = w[vi][0] + 1;
        } else {
            res += w[vi][r as usize] - nt + 1;
            nt = w[vi][r as usize] + 1;
        }
    }

    println!("{}", res);
}
