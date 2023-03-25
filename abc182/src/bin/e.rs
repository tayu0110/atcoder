#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {h: usize, w: usize, n: usize, m: usize, p: [(usize, usize); n], q: [(usize, usize); m]}

    let mut res = vec![vec![0; w]; h];
    for (a, b) in p {
        res[a-1][b-1] = 1;
    }
    for (c, d) in q {
        res[c-1][d-1] = -1;
    }
    let mut res2 = res.clone();
    for i in 0..h {
        for j in 1..w {
            if res[i][j] >= 0 && res[i][j-1] >= 0 {
                res[i][j] += res[i][j-1];
            }
        }
        for j in (0..w-1).rev() {
            if res[i][j] >= 0 && res[i][j+1] >= 0 {
                res[i][j] += res[i][j+1];
            }
        }
    }
    for j in 0..w {
        for i in 1..h {
            if res2[i][j] >= 0 && res2[i-1][j] >= 0 {
                res2[i][j] += res2[i-1][j];
            }
        }
        for i in (0..h-1).rev() {
            if res2[i][j] >= 0 && res2[i+1][j] >= 0 {
                res2[i][j] += res2[i+1][j];
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if res[i][j] > 0 || res2[i][j] > 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
