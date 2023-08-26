use proconio::*;

fn ord(c: u8) -> usize {
    (c - b'a') as usize
}

fn main() {
    input! {s: marker::Bytes, mut k: usize}
    let len = s.len();

    let mut dp = vec![0; len + 1];
    dp[len - 1] = 1;
    let mut next = vec![len; 26];
    next[ord(s[len - 1])] = len - 1;
    for i in (0..len - 1).rev() {
        dp[i] = dp[i + 1] * 2 - dp[next[ord(s[i])]];
        next[ord(s[i])] = i;
    }

    eprintln!("next: {next:?}");
    eprintln!("dp: {dp:?}");
    if dp[0] < k {
        println!("Eel");
        return;
    }

    let mut res = vec![];
    let mut now = 0;
    for _ in 0..len {
        for i in 0..26 {
            if next[i] == len {
                continue;
            }
            if next[i] <= now {
                for j in now + 1..len {
                    if s[next[i]] == s[j] {
                        next[i] = j;
                        break;
                    }
                }
                if next[i] <= now {
                    next[i] = len;
                    continue;
                }
            }

            // eprintln!("i: {i}, now: {now}, next: {}", next[i]);
            if k <= dp[next[i]] {
                now = next[i] + 1;
                res.push((i as u8 + b'a') as char);
                break;
            }
        }
        eprintln!("now: {now}");
    }

    println!("{}", res.iter().collect::<String>())
}
