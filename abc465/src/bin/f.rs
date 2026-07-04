use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize}

    let mut cum = vec![vec![vec![vec![vec![vec![0i64; 11]; 11]; 11]; 11]; 11]; 11];
    for _ in 0..n {
        input! {mut s: marker::Bytes, v: i64};
        s.iter_mut().for_each(|s| *s -= b'0' - 1);
        cum[s[0] as usize][s[1] as usize][s[2] as usize][s[3] as usize][s[4] as usize]
            [s[5] as usize] += v;
    }
    for i in 0..10 {
        for j in 0..11 {
            for k in 0..11 {
                for l in 0..11 {
                    for m in 0..11 {
                        for n in 0..11 {
                            cum[i + 1][j][k][l][m][n] += cum[i][j][k][l][m][n];
                        }
                    }
                }
            }
        }
    }
    for j in 0..10 {
        for i in 0..11 {
            for k in 0..11 {
                for l in 0..11 {
                    for m in 0..11 {
                        for n in 0..11 {
                            cum[i][j + 1][k][l][m][n] += cum[i][j][k][l][m][n];
                        }
                    }
                }
            }
        }
    }
    for k in 0..10 {
        for i in 0..11 {
            for j in 0..11 {
                for l in 0..11 {
                    for m in 0..11 {
                        for n in 0..11 {
                            cum[i][j][k + 1][l][m][n] += cum[i][j][k][l][m][n];
                        }
                    }
                }
            }
        }
    }
    for l in 0..10 {
        for j in 0..11 {
            for k in 0..11 {
                for i in 0..11 {
                    for m in 0..11 {
                        for n in 0..11 {
                            cum[i][j][k][l + 1][m][n] += cum[i][j][k][l][m][n];
                        }
                    }
                }
            }
        }
    }
    for m in 0..10 {
        for j in 0..11 {
            for k in 0..11 {
                for l in 0..11 {
                    for i in 0..11 {
                        for n in 0..11 {
                            cum[i][j][k][l][m + 1][n] += cum[i][j][k][l][m][n];
                        }
                    }
                }
            }
        }
    }
    for n in 0..10 {
        for j in 0..11 {
            for k in 0..11 {
                for l in 0..11 {
                    for m in 0..11 {
                        for i in 0..11 {
                            cum[i][j][k][l][m][n + 1] += cum[i][j][k][l][m][n];
                        }
                    }
                }
            }
        }
    }

    input! {q: usize}
    let mut buf = vec![];
    for _ in 0..q {
        input! {x: marker::Bytes, y: marker::Bytes}
        if x.iter().zip(&y).any(|(x, y)| x > y) {
            buf.push(0);
            continue;
        }

        let resolve = move |i: usize, d: usize| -> usize {
            if i & (1 << d) != 0 {
                (y[d] - b'0' + 1) as usize
            } else {
                (x[d] - b'0') as usize
            }
        };
        let mut ret = 0;
        for i in 0..1 << 6 {
            let d0 = resolve(i, 0);
            let d1 = resolve(i, 1);
            let d2 = resolve(i, 2);
            let d3 = resolve(i, 3);
            let d4 = resolve(i, 4);
            let d5 = resolve(i, 5);
            if i.count_ones() % 2 == 0 {
                ret += cum[d0][d1][d2][d3][d4][d5];
            } else {
                ret -= cum[d0][d1][d2][d3][d4][d5];
            }
        }
        buf.push(ret);
    }
    println!("{}", buf.iter().join("\n"))
}
