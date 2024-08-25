use proconio::*;

fn main() {
    input! {t: marker::Bytes, n: usize}

    let mut dp = vec![i32::MAX; t.len() + 1];
    dp[0] = 0;
    for _ in 0..n {
        input! {a: usize, s: [marker::Bytes; a]}

        for i in (0..t.len()).rev() {
            if dp[i] == i32::MAX {
                continue;
            }
            for s in &s {
                if i + s.len() > t.len() {
                    continue;
                }

                if &t[i..i + s.len()] == &s[..] {
                    dp[i + s.len()] = dp[i + s.len()].min(dp[i] + 1);
                }
            }
        }
    }

    if dp[t.len()] != i32::MAX {
        println!("{}", dp[t.len()])
    } else {
        println!("-1")
    }
}
