use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: [marker::Chars; n]}

    let mut res = vec![vec![-1; m]; n];
    for i in 0..n {
        'b: for j in 0..m {
            for k in (0..=j).rev() {
                if s[i][k] == 'c' {
                    res[i][j] = (j - k) as i32;
                    continue 'b;
                }
            }
        }
    }

    for res in res {
        println!("{}", res.iter().join(" "))
    }
}
