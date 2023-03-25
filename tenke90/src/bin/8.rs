use proconio::input;
use proconio::marker::Chars;

const MOD: i64 = 1000000007;

fn main() {
    input! {n: usize, s: Chars};

    let mut dp = vec![1; n+1];
    let mut buf = "atcoder".to_string();
    while let Some(c) = buf.pop() {
        let mut tmp = vec![0; n+1];
        for i in (0..n).rev() {
            if s[i] == c {
                tmp[i] += dp[i+1];
            }
            tmp[i] += tmp[i+1];
            tmp[i] %= MOD;
        }
        std::mem::swap(&mut dp, &mut tmp);
    }

    println!("{}", dp[0]);
}