use proconio::*;

fn main() {
    input! {h: usize, w: usize, n: usize, p: [(usize, usize); n]}

    let mut cum = vec![vec![0; w + 1]; h + 1];
    for (a, b) in p {
        cum[a][b] += 1;
    }

    for i in 0..h {
        for j in 0..w + 1 {
            cum[i + 1][j] += cum[i][j];
        }
    }
    for j in 0..w {
        for i in 0..h + 1 {
            cum[i][j + 1] += cum[i][j];
        }
    }

    let mut res = 0usize;
    for i in 0..h {
        for j in 0..w {
            let (mut l, mut r) = (i, h + 1);
            while r - l > 1 {
                let s = (r + l) / 2;
                let d = s - i;
                let t = j + d;
                if t > w {
                    r = s;
                    continue;
                }

                if cum[s][t] + cum[i][j] - cum[s][j] - cum[i][t] > 0 {
                    r = s;
                } else {
                    l = s;
                }
            }

            res += l - i;
        }
    }

    println!("{}", res)
}
