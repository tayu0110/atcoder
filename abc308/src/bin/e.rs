use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], s: marker::Chars}

    let mut x = vec![vec![0; 3]; n];
    for (i, &c) in s.iter().enumerate().rev() {
        if c == 'X' {
            x[i][a[i]] += 1;
        }

        if i + 1 < n {
            for j in 0..3 {
                x[i][j] += x[i + 1][j];
            }
        }
    }

    // eprintln!("{x:?}");

    let mut e = vec![vec![vec![0; 3]; 3]; n];
    for (i, &c) in s.iter().enumerate().rev().skip(1) {
        if c == 'E' {
            for j in 0..3 {
                e[i][a[i]][j] += x[i + 1][j];
            }
        }

        for j in 0..3 {
            for k in 0..3 {
                e[i][j][k] += e[i + 1][j][k];
            }
        }
    }
    // eprintln!("{e:?}");

    let mut res = 0usize;
    for (i, &c) in s.iter().enumerate().rev().skip(2) {
        if c == 'M' {
            for j in 0..3 {
                for k in 0..3 {
                    let v = [a[i], j, k];
                    let mut mex = 0;
                    while v.contains(&mex) {
                        mex += 1;
                    }

                    res += mex * e[i][j][k];
                }
            }
        }
    }

    println!("{}", res)
}
