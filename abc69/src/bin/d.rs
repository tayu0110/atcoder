#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};
use itertools::Itertools;

fn main() {
    input! {h: usize, w: usize, n: usize, mut a: [usize; n]}

    let mut now = 0;
    let mut res = vec![vec![0; w]; h];
    for i in 0..h {
        if i % 2 == 0 {
            for j in 0..w {
                res[i][j] = now+1;
                a[now] -= 1;
                while now < n && a[now] == 0 {
                    now += 1;
                }
            }
        } else {
            for j in (0..w).rev() {
                res[i][j] = now+1;
                a[now] -= 1;
                while now < n && a[now] == 0 {
                    now += 1;
                }
            }
        }
    }

    res.iter().for_each(|v| println!("{}", v.iter().join(" ")));
}
