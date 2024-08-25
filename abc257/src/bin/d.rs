use std::collections::VecDeque;

#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, q: [(i64, i64, i64); n]};

    // const INF: i64 = 111222333444555666;
    let mut l = -1;
    let mut r = 5556667778;
    while r - l > 1 {
        let s = (r + l) / 2;
        // let mut dp = vec![vec![INF; n]; n];
        let mut t = vec![vec![]; n];

        for (i, &(x, y, p)) in q.iter().enumerate() {
            // dp[i][i] = 0;
            for (j, &(tx, ty, _)) in q.iter().enumerate() {
                if i == j {
                    continue;
                }

                let m = (x - tx).abs() + (y - ty).abs();
                if p * s >= m {
                    t[i].push(j);
                }
            }
        }

        let mut good = false;
        for i in 0..n {
            let mut nt = VecDeque::new();
            let mut ck = vec![false; n];
            nt.push_back(i);
            while !nt.is_empty() {
                let now = nt.pop_front().unwrap();
                if ck[now] {
                    continue;
                }
                ck[now] = true;
                for to in &t[now] {
                    nt.push_back(*to);
                }
            }

            let res = ck.iter().fold(true, |res, x| res & *x);
            good |= res;
        }

        if good {
            r = s;
        } else {
            l = s;
        }
    }

    println!("{}", r);
}
