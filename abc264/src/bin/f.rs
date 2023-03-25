#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {h: usize, w: usize, r: [usize; h], c: [usize; w], tmp: [Chars; h]};
    let a = tmp
                                .into_iter()
                                .fold(vec![], |mut v, s| {
                                    v.push(s.into_iter().map(|c| (c as u8 - '0' as u8) as usize).collect::<Vec<_>>());
                                    v
                                });
    const INF: usize = 0x3f3f3f3f3f3f3f3f;
    let mut dp = vec![vec![vec![vec![INF; 2]; 2]; w]; h];
    for i in 0..2 {
        for j in 0..2 {
            dp[0][0][i][j] = r[0] * i + c[0] * j;
        }
    }
    for y in 0..h {
        for x in 0..w {
            for &(nrr, ncr) in &[(0, 0), (0, 1), (1, 0), (1, 1)] {
                let color = a[y][x] ^ nrr ^ ncr;
                for &(dx, dy) in &[(0, 1), (1, 0)] {
                    let (tx, ty) = (x + dx, y + dy);
                    if tx >= w || ty >= h {
                        continue;
                    }
                    for &(trr, tcr) in &[(0, 0), (0, 1), (1, 0), (1, 1)] {
                        let ncolor = a[ty][tx] ^ trr ^ tcr;
                        if (x == tx && ncr != tcr) || (y == ty && nrr != trr) || color != ncolor {
                            continue;
                        }
                        let mut new = dp[y][x][nrr][ncr];
                        if y != ty {
                            new += r[ty] * trr;
                        }
                        if x != tx {
                            new += c[tx] * tcr;
                        }
                        dp[ty][tx][trr][tcr] = std::cmp::min(dp[ty][tx][trr][tcr], new);
                    }
                }
            }
        }
    }
    let mut res = INF;
    for i in 0..2 {
        for j in 0..2 {
            res = std::cmp::min(res, dp[h-1][w-1][i][j]);
        }
    }
    println!("{}", res);
}
