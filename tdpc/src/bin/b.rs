use proconio::*;

fn rec(n: usize, m: usize, a: &[usize], b: &[usize], memo: &mut Vec<Vec<usize>>) -> usize {
    if n == a.len() && m == b.len() {
        memo[n][m] = 0;
        return 0;
    }

    if memo[n][m] < std::usize::MAX {
        return memo[n][m];
    }

    let parity = (n + m) % 2;
    let mut res = if parity == 0 { 0 } else { std::usize::MAX };
    if n < a.len() {
        if parity == 0 {
            res = res.max(a[n] + rec(n + 1, m, a, b, memo));
        } else {
            res = res.min(rec(n + 1, m, a, b, memo));
        }
    }

    if m < b.len() {
        if parity == 0 {
            res = res.max(b[m] + rec(n, m + 1, a, b, memo));
        } else {
            res = res.min(rec(n, m + 1, a, b, memo));
        }
    }

    memo[n][m] = res;
    res
}

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; m]}

    let mut memo = vec![vec![std::usize::MAX; m + 1]; n + 1];
    println!("{}", rec(0, 0, &a, &b, &mut memo))
}
