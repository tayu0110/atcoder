use proconio::*;

fn main() {
    input! {_: usize, mut s: marker::Bytes}

    let rem = s
        .iter()
        .cloned()
        .enumerate()
        .filter(|&(_, c)| c != b'?')
        .fold(0, |s, (i, v)| (s + (v - b'0') as usize * (i + 1)) % 10);
    let t = s
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(i, c)| (c == b'?').then_some(i + 1))
        .collect::<Vec<_>>();

    let mut dp = vec![[(u8::MAX, u8::MAX); 10]; t.len() + 1];
    dp[0][0] = (0, 0);
    for (i, &t) in t.iter().enumerate() {
        for j in 0..10 {
            if dp[i][j] == (u8::MAX, u8::MAX) {
                continue;
            }

            for k in 0..10 {
                dp[i + 1][(j + k * t) % 10] = (j as u8, k as u8);
            }
        }
    }

    if dp[t.len()][(10 - rem) % 10] == (u8::MAX, u8::MAX) {
        println!("No");
    } else {
        let mut now = (10 - rem) % 10;
        for (i, t) in t.into_iter().enumerate().rev() {
            let (j, k) = dp[i + 1][now];
            s[t - 1] = b'0' + k;
            now = j as usize;
        }
        println!("Yes");
        println!("{}", s.into_iter().map(|c| c as char).collect::<String>())
    }
}
