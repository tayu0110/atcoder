use proconio::*;
use rustc_hash::FxHashMap;

fn solve(
    now: usize,
    par: usize,
    b: bool,
    a: &[usize],
    t: &[Vec<usize>],
    memo: &mut [bool],
    count: &mut FxHashMap<usize, usize>,
) {
    memo[now] = b | (count.get(&a[now]).copied().unwrap_or_default() > 0);
    *count.entry(a[now]).or_insert(0) += 1;
    for &to in &t[now] {
        if to != par {
            solve(to, now, memo[now], a, t, memo, count);
        }
    }
    *count.entry(a[now]).or_insert(0) -= 1;
}

fn main() {
    input! {n: usize, a: [usize; n], e: [(usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut memo = vec![false; n];
    let mut count = FxHashMap::default();
    solve(0, 0, false, &a, &t, &mut memo, &mut count);
    for f in memo {
        if f { println!("Yes") } else { println!("No") }
    }
}
