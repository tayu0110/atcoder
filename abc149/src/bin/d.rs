#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn solve(t: &Vec<i32>, score: Vec<usize>) -> usize {
    let n = t.len();
    let mut dp = vec![vec![0; 3]; n];
    for i in 0..3 {
        if (t[0]-i+3) % 3 == 1 {
            dp[0][i as usize] = score[i as usize];
        }
    }

    for i in 1..n {
        for j in 0..3 {
            for k in 0..3 {
                if j == k {
                    continue;
                }
                let mut ns = dp[i-1][j];
                if (t[i]-k as i32+3) % 3 == 1 {
                    ns += score[k];
                }
                dp[i][k] = std::cmp::max(dp[i][k], ns);
            }
        }
    }

    *dp[n-1].iter().max().unwrap()
}

fn main() {
    input! {n: usize, k: usize, r: usize, s: usize, p: usize, t: Chars}

    let t = t.into_iter().map(|c| {
        if c == 'r' {
            0i32
        } else if c == 's' {
            1i32
        } else {
            2i32
        }
    }).collect_vec();

    let mut a = vec![vec![]; k];
    for i in 0..n {
        a[i%k].push(t[i]);
    }

    let mut res = 0;
    for i in 0..k {
        res += solve(&a[i], vec![r, s, p]);
    }

    println!("{}", res);
}
