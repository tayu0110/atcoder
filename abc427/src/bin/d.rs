use std::fmt::Write as _;

use proconio::*;

fn solve(
    turn: usize,
    now: usize,
    k: usize,
    s: &[u8],
    t: &[Vec<usize>],
    memo: &mut [Vec<u8>],
) -> bool {
    if k == memo.len() {
        return s[now] == b'A';
    }

    if memo[k][now] != 0 {
        return memo[k][now] == 1;
    }

    let mut ret = false;
    for &to in &t[now] {
        if !solve((turn + 1) % 2, to, k + 1, s, t, memo) {
            ret = true;
        }
    }

    memo[k][now] = 2 - ret as u8;
    ret
}

fn main() {
    input! {t: usize}

    let mut buf = String::new();
    for _ in 0..t {
        input! {n: usize, m: usize, k: usize, s: marker::Bytes, e: [(usize, usize); m]}

        let mut t = vec![vec![]; n];
        for (u, v) in e {
            t[u - 1].push(v - 1);
        }

        // 0: undecided, 1: win, 2: loose
        let mut memo = vec![vec![0u8; n]; k * 2];
        if solve(0, 0, 0, &s, &t, &mut memo) {
            writeln!(buf, "Alice").unwrap();
        } else {
            writeln!(buf, "Bob").unwrap();
        }
    }

    print!("{buf}")
}
