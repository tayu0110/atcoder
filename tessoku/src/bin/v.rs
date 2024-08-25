use proconio::*;

fn rec(now: usize, a: &[usize], b: &[usize], memo: &mut [usize]) -> usize {
    if memo[now] != std::usize::MAX {
        return memo[now];
    }

    memo[now] = (rec(a[now], a, b, memo) + 100).max(rec(b[now], a, b, memo) + 150);
    memo[now]
}

fn main() {
    input! {n: usize, mut a: [usize; n-1], mut b: [usize; n-1]}
    a.iter_mut().for_each(|v| *v -= 1);
    b.iter_mut().for_each(|v| *v -= 1);

    let mut memo = vec![std::usize::MAX; n];
    memo[n - 1] = 0;
    rec(0, &a, &b, &mut memo);
    println!("{}", memo[0])
}
