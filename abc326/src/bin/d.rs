use itertools::Itertools;
use proconio::*;

fn fill(n: usize, now: usize, r: &[char], c: &[char], buf: &mut Vec<Vec<char>>) -> bool {
    if now == n {
        let mut nr = vec![];
        for i in 0..n {
            for j in 0..n {
                if buf[i][j] != '.' {
                    nr.push(buf[i][j]);
                    break;
                }
            }
        }

        if r != nr {
            return false;
        }

        let mut nc = vec![];
        for j in 0..n {
            for i in 0..n {
                if buf[i][j] != '.' {
                    nc.push(buf[i][j]);
                    break;
                }
            }
        }

        if c != nc {
            return false;
        }

        return true;
    }

    for a in 0..n {
        if (0..now).any(|i| buf[i][a] == 'A') {
            continue;
        }
        for b in 0..n {
            if a == b {
                continue;
            }
            if (0..now).any(|i| buf[i][b] == 'B') {
                continue;
            }
            for pc in 0..n {
                if a == pc || b == pc {
                    continue;
                }
                if (0..now).any(|i| buf[i][pc] == 'C') {
                    continue;
                }

                buf[now][a] = 'A';
                buf[now][b] = 'B';
                buf[now][pc] = 'C';

                let f = fill(n, now + 1, r, c, buf);

                if f {
                    return true;
                }

                buf[now][a] = '.';
                buf[now][b] = '.';
                buf[now][pc] = '.';
            }
        }
    }

    false
}

fn main() {
    input! {n: usize, r: marker::Chars, c: marker::Chars}

    let mut buf = vec![vec!['.'; n]; n];
    if fill(n, 0, &r, &c, &mut buf) {
        println!("Yes");
        for v in buf {
            println!("{}", v.iter().join(""))
        }
    } else {
        println!("No")
    }
}
