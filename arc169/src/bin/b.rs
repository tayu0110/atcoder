use proconio::*;

fn solve(n: usize, s: usize, cum: &[usize], memo: &mut [usize]) {
    if cum[n] - cum[0] <= s {
        memo[0] = n;
        return;
    }

    let (mut l, mut r) = (0, n + 1);
    while r - l > 1 {
        let m = (r + l) >> 1;
        if cum[m] - cum[0] <= s {
            l = m;
        } else {
            r = m;
        }
    }

    if memo[l] == usize::MAX {
        solve(n - l, s, &cum[l..], &mut memo[l..]);
    }

    memo[0] = n + memo[l];
}

fn main() {
    input! {n: usize, s: usize, a: [usize; n]}

    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    let mut memo = vec![usize::MAX; n + 1];
    memo[n] = 0;
    for i in 0..n {
        if memo[i] == usize::MAX {
            solve(n - i, s, &cum[i..], &mut memo[i..]);
        }
    }

    println!("{}", memo.iter().sum::<usize>());
}
