use itertools::Itertools;
use proconio::*;

fn dfs(now: usize, a: &[usize], memo: &mut [usize]) -> usize {
    let next = a[now] - 1;
    if next == now {
        memo[now] = next;
        return next;
    }

    if memo[next] == usize::MAX {
        memo[now] = dfs(next, a, memo);
    } else {
        memo[now] = memo[next];
    }
    memo[now]
}

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut memo = vec![usize::MAX; n];
    for i in 0..n {
        dfs(i, &a, &mut memo);
    }

    println!("{}", memo.iter().map(|m| m + 1).join(" "))
}
