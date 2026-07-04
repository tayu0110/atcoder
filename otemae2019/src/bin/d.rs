use proconio::*;

const M: u32 = 1000_000_007;

fn main() {
    input! {n: usize, m: usize, s: [String; m]}

    let mut dp = [[0; 15]; 1001];
    dp[0][0] = 1;
    for i in 0..n {
        let mut next = [[0; 15]; 1001];
        for j in 0..=m {
            for k in 0..15 {
                if dp[j][k] == 0 {
                    continue;
                }
                for l in 0..10 {
                    if i == 0 && l == 0 {
                        continue;
                    }

                    let nk = (k * 10 + l) % 15;
                    if nk % 3 == 0 && nk % 5 == 0 {
                        if j < m && s[j] == "FizzBuzz" {
                            next[j + 1][nk] += dp[j][k];
                            next[j + 1][nk] %= M;
                        }
                    } else if nk % 3 == 0 {
                        if j < m && s[j] == "Fizz" {
                            next[j + 1][nk] += dp[j][k];
                            next[j + 1][nk] %= M;
                        }
                    } else if nk % 5 == 0 {
                        if j < m && s[j] == "Buzz" {
                            next[j + 1][nk] += dp[j][k];
                            next[j + 1][nk] %= M;
                        }
                    } else {
                        next[j][nk] += dp[j][k];
                        next[j][nk] %= M;
                    }
                }
            }
        }
        dp = next;
    }

    println!("{}", dp[m].iter().fold(0, |s, v| (s + v) % M))
}
