use std::fmt::Write as _;

use proconio::*;

fn main() {
    input! {n: usize, q: usize, a: [usize; n]}

    let mut dest = vec![vec![0; n]; 30];
    for i in 0..n {
        dest[0][i] = a[i] - 1;
    }
    for i in 0..29 {
        for j in 0..n {
            dest[i + 1][j] = dest[i][dest[i][j]];
        }
    }

    let mut amount = vec![vec![0; n]; 30];
    for i in 0..n {
        amount[0][i] = i + 1;
    }
    for i in 0..29 {
        for j in 0..n {
            amount[i + 1][j] = amount[i][j] + amount[i][dest[i][j]];
        }
    }

    let mut buf = String::new();
    for _ in 0..q {
        input! {t: usize, mut b: usize}
        b -= 1;
        let mut ret = 0;
        for i in (0..30).rev() {
            if t & (1 << i) != 0 {
                ret += amount[i][b];
                b = dest[i][b];
            }
        }

        writeln!(buf, "{ret}").unwrap();
    }
    print!("{buf}")
}
