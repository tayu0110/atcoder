use math::MathInt;
use proconio::*;

const M: usize = 998244353;

fn dfs(now: usize, t: &[Vec<usize>], c: &[usize], d: &[usize], memo: &mut [usize]) -> usize {
    let mut sum = 0;
    for &to in &t[now] {
        sum += dfs(to, t, c, d, memo);
    }

    sum += c[now];
    if sum < d[now] {
        memo[now] = usize::MAX;
        0
    } else {
        memo[now] = sum;
        sum - d[now]
    }
}

fn main() {
    input! {n: usize, p: [usize; n - 1], c: [usize; n], d: [usize; n]}

    let mut t = vec![vec![]; n];
    for (i, p) in p.into_iter().enumerate() {
        t[p - 1].push(i + 1);
    }

    let mut memo = vec![0; n];
    dfs(0, &t, &c, &d, &mut memo);
    if memo.iter().any(|&m| m == usize::MAX) {
        println!("0");
        return;
    }

    let mut ret = 1;
    for i in 0..n {
        let now = memo[i];
        let d = d[i];

        let mut num = 1;
        let mut den = 1;
        for j in 0..d {
            num *= (now - j) % M;
            den *= j + 1;

            num %= M;
            den %= M;
        }
        ret *= num;
        ret %= M;

        // eprintln!("num: {num}, den: {den}");
        ret *= den.inverse_mod(M).unwrap();
        ret %= M;
    }

    println!("{ret}")
}
