#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, m: usize, p: [(usize, usize); n]};

    let mut t = vec![vec![]; m+1];
    for (i, (v, w)) in p.into_iter().enumerate() {
        t[v].push(i);
        t[w].push(i);
    }

    let mut l = 1;
    let mut r = 1;
    let mut cnt = n;
    let mut v = vec![0; n];
    let mut res = vec![0i64; m+3];
    while l <= m {
        while r <= m && cnt != 0 {
            for w in &t[r] {
                if v[*w] == 0 {
                    cnt -= 1;
                }
                v[*w] += 1;
            }
            r += 1;
        }

        if cnt != 0 {
            break;
        }

        res[r - l] += 1;
        res[m - l + 2] -= 1;
        for w in &t[l] {
            v[*w] -= 1;
            if v[*w] == 0 {
                cnt += 1;
            }
        }
        l += 1;
    }
    for i in 1..=m {
        if i > 1 {
            print!(" ");
        }
        res[i] += res[i-1];
        print!("{}", res[i]);
    }
    println!();
}
