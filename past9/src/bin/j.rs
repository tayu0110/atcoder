use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut buf = vec![vec![0u8; n]; n];
    let (mut u, mut r) = (0, 1);
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {x: usize, y: usize}
            let (mut x, mut y) = (x - 1, y - 1);
            for _ in 0..4 - u {
                (x, y) = (y, n - 1 - x);
            }

            if (u + 1) % 4 != r {
                y = n - 1 - y;
            }
            buf[x][y] ^= 1;
        } else if ty == 2 {
            input! {c: char}
            if c == 'A' {
                u = (u + 1) % 4;
                r = (r + 1) % 4;
            } else {
                u = (u + 3) % 4;
                r = (r + 3) % 4;
            }
        } else {
            input! {c: char}
            if c == 'A' {
                if u % 2 == 0 {
                    u = (u + 2) % 4;
                } else {
                    r = (r + 2) % 4;
                }
            } else {
                if u % 2 == 1 {
                    u = (u + 2) % 4;
                } else {
                    r = (r + 2) % 4;
                }
            }
        }
    }

    let mut new = vec![vec![0; n]; n];
    for _ in 0..u {
        for i in 0..n {
            for j in 0..n {
                new[j][n - 1 - i] = buf[i][j];
            }
        }
        (buf, new) = (new, buf);
    }
    if (u + 1) % 4 != r {
        if r % 2 == 1 {
            buf.iter_mut().for_each(|b| b.reverse());
        } else {
            buf.reverse();
        }
    }

    println!("{}", buf.iter().map(|b| b.iter().join("")).join("\n"))
}
