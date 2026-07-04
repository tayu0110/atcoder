use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, mut s: marker::Bytes}
        s.insert(0, b'0');

        let mut dp = vec![false; 1 << n];
        dp[0] = true;
        for i in 0..1 << n {
            if !dp[i] || s[i] == b'1' {
                continue;
            }
            for j in 0..n {
                if i & (1 << j) == 0 {
                    let next = i | (1 << j);
                    if s[next] == b'0' {
                        dp[next] = true;
                    }
                }
            }
        }

        if dp[(1 << n) - 1] {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
