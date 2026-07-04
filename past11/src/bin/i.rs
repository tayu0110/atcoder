use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, mut s: [marker::Bytes; h]}

    let (mut ai, mut aj) = (0, 0);
    let (mut si, mut sj) = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'a' {
                (ai, aj) = (i, j);
                s[i][j] = b'.';
            } else if s[i][j] == b's' {
                (si, sj) = (i, j);
                s[i][j] = b'.';
            }
        }
    }

    let mut memo = [[[[u16::MAX; 50]; 50]; 50]; 50];
    memo[si][sj][ai][aj] = 0;
    let mut nt = VecDeque::new();
    nt.push_back((si, sj, ai, aj));
    while let Some((si, sj, ai, aj)) = nt.pop_front() {
        for (di, dj) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let ni = si.wrapping_add(di);
            let nj = sj.wrapping_add(dj);
            if ni < h && nj < w {
                if (ni, nj) == (ai, aj) {
                    let nai = ai.wrapping_add(di);
                    let naj = aj.wrapping_add(dj);
                    if nai < h && naj < w && s[nai][naj] != b'#' {
                        if s[nai][naj] == b'g' {
                            println!("{}", memo[si][sj][ai][aj] + 1);
                            return;
                        }
                        if memo[ni][nj][nai][naj] == u16::MAX {
                            memo[ni][nj][nai][naj] = memo[si][sj][ai][aj] + 1;
                            nt.push_back((ni, nj, nai, naj));
                        }
                    }
                } else if s[ni][nj] == b'.' || s[ni][nj] == b'g' {
                    if memo[ni][nj][ai][aj] == u16::MAX {
                        memo[ni][nj][ai][aj] = memo[si][sj][ai][aj] + 1;
                        nt.push_back((ni, nj, ai, aj));
                    }
                }
            }
        }
    }
    println!("-1");
}
