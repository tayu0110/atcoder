use proconio::*;

fn main() {
    input! {k: usize, s: marker::Bytes, t: marker::Bytes}

    let mut dp = vec![[[u8::MAX; 21]; 21]; s.len() + 1];
    dp[0][0][0] = 0;
    for i in 0..=s.len() {
        for add in 0..=k {
            for del in 0..=k - add {
                if dp[i][add][del] == u8::MAX {
                    continue;
                }

                if i + add < del {
                    continue;
                }

                if i + add - del < t.len() && i < s.len() {
                    if s[i] == t[i + add - del] {
                        dp[i + 1][add][del] = dp[i + 1][add][del].min(dp[i][add][del]);
                    } else {
                        dp[i + 1][add][del] =
                            dp[i + 1][add][del].min(dp[i][add][del].saturating_add(1));
                    }
                }

                if add + del + 1 <= k {
                    dp[i][add + 1][del] = dp[i][add + 1][del].min(dp[i][add][del]);
                    if i < s.len() {
                        dp[i + 1][add][del + 1] = dp[i + 1][add][del + 1].min(dp[i][add][del]);
                    }
                }
            }
        }
    }

    for i in 0..=k {
        for j in 0..=k {
            if s.len() + i < j {
                continue;
            }

            if s.len() + i - j != t.len() {
                continue;
            }

            if i + j + dp[s.len()][i][j] as usize <= k {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
