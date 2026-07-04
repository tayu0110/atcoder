use std::iter::once;

use proconio::*;

fn solve(now: usize, today: usize, s: &[Vec<u8>], t: &[Vec<usize>], memo: &mut [Vec<i8>]) -> bool {
    memo[today][now] = -2;
    let w = s[0].len();
    let tomorrow = (today + 1) % w;
    for to in t[now]
        .iter()
        .copied()
        .chain(once(now))
        .filter(|&to| s[to][tomorrow] == b'o')
    {
        if memo[tomorrow][to] == -2 {
            memo[today][now] = 1;
            return true;
        }
        if memo[tomorrow][to] >= 0 {
            continue;
        }
        if solve(to, tomorrow, s, t, memo) {
            memo[today][now] = 1;
            return true;
        }
    }
    memo[today][now] = 0;
    false
}

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, m: usize, e: [(usize, usize); m], w: usize, s: [marker::Bytes; n]}

        let mut t = vec![vec![]; n];
        for (u, v) in e {
            t[u - 1].push(v - 1);
            t[v - 1].push(u - 1);
        }

        // -1: not yet, -2: serching, 0: bad, 1: good
        let mut memo = vec![vec![-1i8; n]; w];
        let mut ok = false;
        for i in 0..n {
            if memo[0][i] >= 0 {
                continue;
            }
            if s[i][0] == b'x' {
                continue;
            }
            if solve(i, 0, &s, &t, &mut memo) {
                ok = true;
                break;
            }
        }

        if ok { println!("Yes") } else { println!("No") }
    }
}
