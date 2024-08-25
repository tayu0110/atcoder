use proconio::*;

const M: usize = 998244353;

fn solve_single(s: &[u8]) -> usize {
    let mut last = [usize::MAX; 128];
    let mut dp = vec![0; s.len()];
    for (i, &c) in s.iter().enumerate() {
        for pc in b'a'..=b'z' {
            dp[i] += *dp.get(last[pc as usize]).unwrap_or(&1);
            dp[i] %= M;
        }
        last[c as usize] = i;
    }

    *dp.last().unwrap()
}

fn main() {
    input! {s: marker::Bytes, t: marker::Bytes}

    let mut res = solve_single(&s);
    res += solve_single(&t);
}
