use std::io::stdin;

use cpio::*;

fn main() {
    let mut s = stdin().lines();
    s.next();

    while let Some(_) = s.next() {
        let s = s.next().unwrap().unwrap();
        let s = s.as_bytes();
        let mut dp = [0; 3];
        dp[0] = (s[0] == b'1') as u32;
        dp[1] = (s[0] == b'0') as u32;
        dp[2] = (s[0] == b'1') as u32;

        for &s in s.iter().skip(1) {
            dp[2] = (dp[1] + (s == b'1') as u32).min(dp[2] + (s == b'1') as u32);
            dp[1] = (dp[0] + (s == b'0') as u32).min(dp[1] + (s == b'0') as u32);
            dp[0] = dp[0] + (s == b'1') as u32;
        }

        putln!(dp[0].min(dp[1]).min(dp[2]));
    }
}
