#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, s: usize, p: [(usize, usize); n]}

    let mut dp = vec![vec![std::usize::MAX; s+1]; n+1];
    dp[0][0] = 0;

    for (i, &(a, b)) in p.iter().enumerate() {
        for j in (0..=s).rev() {
            if dp[i][j] == std::usize::MAX {
                continue;
            }

            if j + a <= s {
                dp[i+1][j+a] = 0;
            }

            if j + b <= s {
                dp[i+1][j+b] = 1;
            }
        }
    }

    let mut now = s;
    let mut res = vec![];
    for i in (1..=n).rev() {
        if dp[i][now] == std::usize::MAX {
            println!("No");
            std::process::exit(0);
        }

        if dp[i][now] == 0 {
            res.push('H');
            now -= p[i-1].0;
        } else {
            res.push('T');
            now -= p[i-1].1;
        }
    }

    res.reverse();
    let res = res.into_iter().collect::<String>();
    println!("Yes");
    println!("{}", res);
}
