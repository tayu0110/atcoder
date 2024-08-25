use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], p: [(usize, usize, usize, usize); m]}

    let mut bf = vec![vec![0; 3]; n];
    let mut bb = vec![vec![0; 3]; n];
    for (p, q, l, r) in p {
        let q = q - 1;
        if q < n {
            bb[q][r] += p;
        }
        if q > 0 {
            bf[q - 1][l] += 1;
        }
    }

    let mut f = vec![vec![0; 3]; n + 1];
    for i in 0..n {
        for j in 0..3 {
            if f[i][j] == 0 && !(i == 0 && j == 0) {
                continue;
            }

            f[i + 1][j] = f[i + 1][j].max(f[i][j] + bf[i][j]);
            f[i + 1][(j + 1) % 3] = f[i + 1][(j + 1) % 3].max(f[i][j] + a[i] + bf[i][(j + 1) % 3]);
        }
    }
    eprintln!("f: {f:?}");

    let mut b = vec![vec![0; 3]; n + 1];
    for i in (1..n + 1).rev() {
        for j in 0..3 {
            if b[i][j] == 0 && !(i == n && j == 0) {
                continue;
            }

            b[i - 1][j] = b[i - 1][j].max(b[i][j] + bb[i - 1][j]);
            b[i - 1][(j + 1) % 3] =
                b[i - 1][(j + 1) % 3].max(b[i][j] + a[i - 1] + bb[i - 1][(j + 1) % 3]);
        }
    }
    eprintln!("b: {b:?}");

    let mut res = 0;
    for i in 0..n {
        for j in 0..3 {
            for k in 0..3 {
                res = res.max(f[i][j] + b[i + 1][k]);
            }
        }
    }

    println!("{res}")
}
