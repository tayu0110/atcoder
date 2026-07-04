use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [[usize; n]; n]}

    let mut b = vec![vec![0; n + 2]; n + 2];
    for i in 0..n {
        for j in 0..n {
            b[i + 1][j + 1] = a[i][j];
        }
    }

    let mut res = 0;
    let mut buf = [0; 4];
    for i in 1..n {
        for j in 1..n {
            for k in 0..2 {
                for l in 0..2 {
                    buf[k * 2 + l] = b[i + k][j + l];
                }
            }
            buf.sort_unstable();
            if buf.windows(2).filter(|v| v[0] == v[1]).count() <= 1 {
                res += 1;
            }
        }
    }

    let k = k.min(20);
    let mut max = res;
    for i in 1..=n {
        for j in 1..=n {
            for replace in 1..=k {
                let old = b[i][j];
                let mut tmp = res;
                for k in i - 1..=i {
                    for l in j - 1..=j {
                        if k == 0 || l == 0 || k == n || l == n {
                            continue;
                        }
                        for dk in 0..2 {
                            for dl in 0..2 {
                                buf[dk * 2 + dl] = b[k + dk][l + dl];
                            }
                        }
                        buf.sort_unstable();
                        if buf.windows(2).filter(|v| v[0] == v[1]).count() <= 1 {
                            tmp -= 1;
                        }
                        b[i][j] = replace;
                        for dk in 0..2 {
                            for dl in 0..2 {
                                buf[dk * 2 + dl] = b[k + dk][l + dl];
                            }
                        }
                        buf.sort_unstable();
                        if buf.windows(2).filter(|v| v[0] == v[1]).count() <= 1 {
                            tmp += 1;
                        }
                        b[i][j] = old;
                    }
                }
                max = max.max(tmp);
            }
        }
    }

    println!("{max}")
}
