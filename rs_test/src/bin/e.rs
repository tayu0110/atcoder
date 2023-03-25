#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {h: usize, w: usize, s: [Chars; h]}

    let mut v = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                v[i][j] += 1;
                if j > 0 {
                    v[i][j] += v[i][j-1];
                }
            }
        }
        for j in (0..w).rev() {
            if j < w-1 && s[i][j] == '.' && s[i][j+1] == '.' {
                v[i][j] = v[i][j+1];
            }
        }
    }

    let mut y = vec![vec![0; w]; h];
    for j in 0..w {
        for i in 0..h {
            if s[i][j] == '.' {
                y[i][j] += 1;
                if i > 0 {
                    y[i][j] += y[i-1][j];
                }
            }
        }
        for i in (0..h).rev() {
            if i < h-1 && s[i][j] == '.' && s[i+1][j] == '.' {
                y[i][j] = y[i+1][j];
            }
        }
    }

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                res = std::cmp::max(res, v[i][j] + y[i][j] - 1);
            }
        }
    }

    println!("{}", res);
}
