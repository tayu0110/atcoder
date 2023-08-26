use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, s: marker::Chars}
    let s = s
        .into_iter()
        .map(|c| (c == 'B') as usize)
        .collect::<Vec<_>>();

    let mut ones = vec![];
    for (i, &c) in s.iter().enumerate() {
        if c == 1 {
            ones.push(i);
        }
    }
    let cnt = ones.len();

    let mut dp = vec![vec![Modint::zero(); n + 1]; cnt];
    for i in 1..=ones[0] + 1 {
        dp[0][i] = Modint::one();
    }
    for i in 1..cnt {
        let prev_len = ones[i - 1] + 1;
        let now_len = ones[i] + 1;
        for j in 1..=prev_len {
            let now = dp[i - 1][j];
            for k in j + 1..=now_len {
                let diff = k - j;
                dp[i][k] += now * Modint::raw(diff as u32);
            }
        }
    }
    eprintln!("dp: {dp:?}");

    println!("{}", dp[cnt - 1][n])
}
