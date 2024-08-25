use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes, c: [usize; n]}

    let mut f = vec![vec![0; 2]; n];
    let mut b = vec![vec![0; 2]; n];

    for (i, &s) in s.iter().enumerate() {
        if s == b'0' {
            f[i][1] += c[i];
        } else {
            f[i][0] += c[i];
        }

        if i > 0 {
            f[i][0] += f[i - 1][1];
            f[i][1] += f[i - 1][0];
        }
    }

    for (i, &s) in s.iter().enumerate().rev() {
        if s == b'0' {
            b[i][1] += c[i];
        } else {
            b[i][0] += c[i];
        }

        if i + 1 < n {
            b[i][0] += b[i + 1][1];
            b[i][1] += b[i + 1][0];
        }
    }

    let mut res = usize::MAX;
    for i in 0..n - 1 {
        res = res.min(f[i][0] + b[i + 1][0]);
        res = res.min(f[i][1] + b[i + 1][1]);
    }

    println!("{res}")
}
