use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, na: usize, nb: usize, nc: usize, nd: usize, a: [usize; na], b: [usize; nb], c: [usize; nc], d: [usize; nd]}

    let mut dp =
        vec![vec![vec![vec![[u8::MAX; 4]; d.len() + 1]; c.len() + 1]; b.len() + 1]; a.len() + 1];
    let result = [a, b, c, d];
    dp[0][0][0][0] = [0; 4];
    for i in 0..=na {
        for j in 0..=nb {
            for k in 0..=nc {
                for l in 0..=nd {
                    if dp[i][j][k][l] == [u8::MAX; 4] {
                        continue;
                    }

                    for m in 0..4 {
                        let mut index = [i, j, k, l];
                        let mut buf = Vec::with_capacity(3);
                        for n in 0..4 {
                            if m == n {
                                continue;
                            }

                            if result[n].len() == index[n] {
                                continue;
                            }

                            buf.push(result[n][index[n]]);
                            index[n] += 1;
                        }

                        buf.sort();
                        buf.dedup();

                        if buf == vec![1, 2, 3] {
                            dp[index[0]][index[1]][index[2]][index[3]] =
                                [i as u8, j as u8, k as u8, l as u8];
                        }
                    }
                }
            }
        }
    }

    let mut now = [na as u8, nb as u8, nc as u8, nd as u8];
    let mut res = vec![];
    while now != [0; 4] {
        let next = dp[now[0] as usize][now[1] as usize][now[2] as usize][now[3] as usize];
        if next == [u8::MAX; 4] {
            println!("No");
            return;
        }

        for i in 0..4 {
            if now[i] == next[i] {
                res.push(i + 1);
                break;
            }
        }

        now = next;
    }

    if res.len() != n {
        println!("No");
        return;
    }

    res.reverse();
    println!("Yes");
    println!("{}", res.iter().join("\n"));
}
