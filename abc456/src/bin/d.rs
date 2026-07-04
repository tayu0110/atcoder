use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {mut s: marker::Bytes}

    let n = s.len();
    for i in 0..n {
        s[i] -= b'a';
    }
    let mut dp = vec![vec![0; n + 1]; 4];
    let mut cum = vec![vec![0; n + 1]; 4];
    dp[3][0] += 1;
    cum[3][0] += 1;

    for j in 0..n {
        for i in 0..4 {
            if i == s[j] {
                continue;
            }

            dp[s[j] as usize][j + 1] += cum[i as usize][j];
            dp[s[j] as usize][j + 1] %= M;
        }
        for i in 0..4 {
            cum[i][j + 1] = cum[i][j] + dp[i][j + 1];
            cum[i][j + 1] %= M;
        }
    }
    // eprintln!("dp: {dp:?}, cum: {cum:?}");
    println!("{}", cum.iter().take(3).map(|c| c[n]).sum::<usize>() % M)
}
