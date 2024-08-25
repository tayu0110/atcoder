use proconio::input;

const MOD: usize = 1_000_000_007;

fn rec(now: usize, par: usize, t: &Vec<Vec<usize>>, memo: &mut Vec<Vec<usize>>) {
    memo[now] = vec![1; 2];
    for &to in &t[now] {
        if to == par {
            continue;
        }

        rec(to, now, t, memo);

        memo[now][0] *= memo[to][0] + memo[to][1];
        memo[now][0] %= MOD;
        memo[now][1] *= memo[to][0];
        memo[now][1] %= MOD;
    }
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (x, y) in p {
        t[x - 1].push(y - 1);
        t[y - 1].push(x - 1);
    }

    // 0: white, 1: black
    let mut memo = vec![vec![0; 2]; n];
    rec(0, 0, &t, &mut memo);

    println!("{}", (memo[0][0] + memo[0][1]) % MOD)
}
