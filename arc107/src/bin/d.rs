use proconio::*;

const MOD: usize = 998244353;

fn rec(n: usize, k: usize, memo: &mut Vec<Vec<usize>>) -> usize {
    if n < k {
        return 0;
    }

    if k == 0 {
        memo[n][k] = (n == 0) as usize;
        return (n == 0) as usize;
    }

    if memo[n][k] != std::usize::MAX {
        return memo[n][k];
    }

    memo[n][k] = rec(n - 1, k - 1, memo) + rec(n, 2 * k, memo);
    memo[n][k] %= MOD;
    memo[n][k]
}

fn main() {
    input! {n: usize, k: usize}

    let mut memo = vec![vec![std::usize::MAX; 2 * n + 1]; n + 1];
    println!("{}", rec(n, k, &mut memo))
}
