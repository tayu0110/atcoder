#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {h: usize, w: usize, mut a: [[usize; w]; h]}

    let mut res = vec![];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] % 2 == 1 {
                if j+1 < w {
                    a[i][j+1] += 1;
                    a[i][j] -= 1;
                    res.push((i+1, j+1, i+1, j+2));
                } else if i+1 < h {
                    a[i+1][j] += 1;
                    a[i][j] -= 1;
                    res.push((i+1, j+1, i+2, j+1));
                }
            }
        }
    }

    println!("{}", res.len());
    for (a, b, c, d) in res {
        println!("{} {} {} {}", a, b, c, d);
    }
}
