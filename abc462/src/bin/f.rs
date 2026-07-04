use cpio::{putln, scan};

const BASE: usize = 2;

fn main() {
    scan!(t: usize);
    for _ in 0..t {
        scan!(mut s: String, k: usize);
        let s = unsafe { s.as_bytes_mut() };

        let n = s.len();
        let cnt = s.windows(3).filter(|v| v == b"ABC").count();
        if (cnt + k) * 3 > n {
            putln!(-1);
            continue;
        }
        s.iter_mut().for_each(|s| *s = (*s - b'A').min(3));

        let mut dp = vec![vec![vec![vec![u32::MAX; n]; k + 3 + BASE]; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    let diff = [i, j, k]
                        .into_iter()
                        .zip(&s[..3])
                        .filter(|(s, t)| *s != **t)
                        .count();
                    if diff == 0 {
                        dp[j as usize][k as usize][BASE][2] = diff as u32;
                    } else if s[..3] == [0, 1, 2] {
                        dp[j as usize][k as usize][BASE - 1][2] = diff as u32;
                    } else if [i, j, k] == [0, 1, 2] {
                        dp[j as usize][k as usize][BASE + 1][2] = diff as u32;
                    } else {
                        dp[j as usize][k as usize][BASE][2] = diff as u32;
                    }
                }
            }
        }

        for i in 3..n {
            let s = &s[i - 2..=i];
            let f = s == [0, 1, 2];
            for b1 in 0..4 {
                for b2 in 0..4 {
                    let f2 = b1 == 0 && b2 == 1;
                    for c in 0..BASE + k + 3 {
                        if dp[b1][b2][c][i - 1] == u32::MAX {
                            continue;
                        }

                        for b3 in 0..4 {
                            let d = (s[2] != b3 as u8) as u32;
                            let diff = (b1 as u8 != s[0]) as usize
                                + (b2 as u8 != s[1]) as usize
                                + d as usize;
                            if diff == 0 {
                                dp[b2][b3][c][i] = dp[b2][b3][c][i].min(dp[b1][b2][c][i - 1]);
                            } else if f {
                                if c > 0 {
                                    dp[b2][b3][c - 1][i] =
                                        dp[b2][b3][c - 1][i].min(dp[b1][b2][c][i - 1] + d);
                                }
                            } else if f2 && b3 == 2 {
                                if c < BASE + k + 2 {
                                    dp[b2][b3][c + 1][i] =
                                        dp[b2][b3][c + 1][i].min(dp[b1][b2][c][i - 1] + d);
                                }
                            } else {
                                dp[b2][b3][c][i] = dp[b2][b3][c][i].min(dp[b1][b2][c][i - 1] + d);
                            }
                        }
                    }
                }
            }
        }

        let mut ret = u32::MAX;
        for i in 0..4 {
            for j in 0..4 {
                ret = ret.min(dp[i][j][BASE + k][n - 1]);
            }
        }
        putln!(ret);
    }
}
