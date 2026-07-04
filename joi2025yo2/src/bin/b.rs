use proconio::*;

fn dfs(
    now: usize,
    x: usize,
    next: &[i32],
    a: &[usize],
    memo: &mut [(usize, usize)],
    checking: &mut [bool],
) -> (usize, usize) {
    if memo[now].0 < usize::MAX {
        return memo[now];
    }
    if checking[now] {
        memo[now] = (0, usize::MAX);
        return (0, usize::MAX);
    }
    checking[now] = true;

    if next[now] > 0 {
        let (max, sum) = dfs(next[now] as usize - 1, x, next, a, memo, checking);
        if sum.saturating_add(a[now]) <= x {
            memo[now] = (max.max(now + 1), sum + a[now]);
        } else {
            memo[now] = (max, sum.saturating_add(a[now]));
        }
    } else {
        if a[now] <= x {
            memo[now] = (now + 1, a[now]);
        } else {
            memo[now] = (0, a[now]);
        }
    }
    checking[now] = false;
    memo[now]
}

fn main() {
    input! {n: usize, x: usize, a: [usize; n], p: [i32; n]}

    let mut checking = vec![false; n];
    let mut memo = vec![(usize::MAX, usize::MAX); n];
    let mut res = 0;
    for i in 0..n {
        if memo[i].0 < usize::MAX {
            res = res.max(memo[i].0);
            continue;
        }
        dfs(i, x, &p, &a, &mut memo, &mut checking);
        res = res.max(memo[i].0);
    }

    if res > 0 {
        println!("{res}")
    } else {
        println!("-1")
    }
}
