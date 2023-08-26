use proconio::*;

fn rec(now: usize, max: usize, s: &[char]) -> Option<usize> {
    if now == s.len() {
        return Some(0);
    }

    let exp = s.len() - 1 - now;
    if s[now] != '?' {
        let k = (s[now] as usize - b'0' as usize) << exp;
        if k > max {
            return None;
        }
        if let Some(res) = rec(now + 1, max - k, s) {
            return Some(res + k);
        } else {
            return None;
        }
    }

    if 1usize << exp <= max {
        if let Some(res) = rec(now + 1, max - (1usize << exp), s) {
            return Some(res + (1usize << exp));
        }
    }

    if let Some(res) = rec(now + 1, max, s) {
        return Some(res);
    }

    None
}

fn main() {
    input! {s: marker::Chars, n: usize}

    if let Some(res) = rec(0, n, &s) {
        println!("{}", res);
    } else {
        println!("-1")
    }
}
