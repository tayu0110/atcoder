use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, p: usize, q: usize, e: [(usize, usize); p], re: [(usize, usize); q]}

    let mut t = vec![vec![]; n + 1];
    for (a, b) in e {
        t[a].push(b);
        t[b].push(a);
    }

    for (c, d) in re {
        t[c].push(d.wrapping_neg());
        t[d].push(c.wrapping_neg());
    }

    let mut res = vec![usize::MAX; n + 1];
    let mut stack = vec![];
    let mut reached = vec![false; n];
    let mut mex = vec![];
    let mut memo = vec![];
    for i in 1..n + 1 {
        if res[i] == usize::MAX {
            stack.push(i);
            while let Some(now) = stack.pop() {
                for &to in &t[now] {
                    if to <= n {
                        if !reached[to] {
                            stack.push(to);
                            memo.push(to);
                            reached[to] = true;
                        }
                    } else {
                        mex.push(res[to.wrapping_neg()]);
                    }
                }
            }
            mex.sort_unstable();
            mex.dedup();
            for i in 0..=mex.len() {
                if i == mex.len() || i + 1 != mex[i] {
                    for m in memo.drain(..) {
                        res[m] = i + 1;
                    }
                    break;
                }
            }
        }
    }

    println!("{}", res[1..].iter().join(" "));
}
