use std::collections::VecDeque;

use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut res = vec![usize::MAX; n];
    let mut nt = VecDeque::new();
    let mut buf = vec![];
    for i in 0..n {
        if res[i] == usize::MAX {
            res[i] = 1;
            nt.push_back(i);
            let mut s = [0; 3];
            let mut bad = false;
            while let Some(now) = nt.pop_front() {
                s[res[now]] += 1;
                for &to in &t[now] {
                    if res[to] == usize::MAX {
                        res[to] = 3 - res[now];
                        nt.push_back(to);
                    } else {
                        bad |= (res[now] + res[to]) % 3 != 0;
                    }
                }
            }
            if bad {
                buf.push(([n + 1; 3], [s[1] + s[2], 0, 0]));
            } else {
                buf.push((s, [s[1] + s[2], 0, 0]));
            }
        }
    }

    let mut memo = FxHashSet::default();
    memo.insert((0, 0, 0));
    let f = [n / 3, (n + 2) / 3, n / 3 + (n % 3 == 2) as usize];
    let mut new = FxHashSet::default();
    for (x, y) in buf {
        new.clear();
        for &(s, t, u) in &memo {
            for b in [x, [0, x[2], x[1]], y] {
                let s = s + b[0];
                let t = t + b[1];
                let u = u + b[2];
                if s <= f[0] && t <= f[1] && u <= f[2] {
                    new.insert((s, t, u));
                }
            }
        }
        (memo, new) = (new, memo);
    }

    if !memo.is_empty() {
        println!("Yes")
    } else {
        println!("No")
    }
}
