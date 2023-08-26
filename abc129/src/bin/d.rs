use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Chars; h]}

    let mut r = vec![vec![0; w]; h];
    let mut c = r.clone();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }

            if j > 0 {
                c[i][j] = c[i][j - 1] + 1;
            } else {
                c[i][j] = 1;
            }

            if i > 0 {
                r[i][j] = r[i - 1][j] + 1;
            } else {
                r[i][j] = 1;
            }
        }
    }
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if s[i][j] == '#' {
                continue;
            }

            if i + 1 < h {
                r[i][j] = r[i][j].max(r[i + 1][j]);
            }
            if j + 1 < w {
                c[i][j] = c[i][j].max(c[i][j + 1]);
            }
        }
    }

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' {
                res = res.max(r[i][j] + c[i][j] - 1);
            }
        }
    }

    println!("{}", res)
}
