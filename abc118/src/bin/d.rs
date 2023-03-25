#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

const M: [usize; 10] = [0, 2, 5, 5, 4, 5, 6, 3, 7, 6];

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    let mut dp = vec![vec![None; 10]; n + 1];
    dp[0][0] = Some("".to_string());

    for i in 0..n {
        for j in 0..10 {
            if let Some(mut s) = dp[i][j].clone() {
                for k in 0..m {
                    if i + M[a[k]] > n {
                        continue;
                    }

                    s.push((b'0' + a[k] as u8) as char);
                    if let Some(t) = dp[i + M[a[k]]][k].clone() {
                        if s.len() > t.len() {
                            dp[i + M[a[k]]][k] = Some(s.clone());
                        } else if s.len() == t.len() && s > t {
                            dp[i + M[a[k]]][k] = Some(s.clone());
                        }
                    } else {
                        dp[i + M[a[k]]][k] = Some(s.clone());
                    }
                    s.pop().unwrap();
                }
            }
        }
    }

    let mut res = "".to_string();
    for i in 0..10 {
        if let Some(s) = dp[n][i].clone() {
            if s.len() > res.len() {
                res = s;
            } else if s.len() == res.len() && s > res {
                res = s;
            }
        }
    }

    println!("{}", res);
}
