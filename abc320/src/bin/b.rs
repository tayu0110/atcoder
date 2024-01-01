use proconio::*;

fn is_palindrome(s: &[char]) -> bool {
    let len = s.len();
    for l in 0..len {
        let r = len - l - 1;
        if s[l] != s[r] {
            return false;
        }
    }
    true
}

fn main() {
    input! {s: marker::Chars}
    let n = s.len();

    let mut res = 0;
    for i in 0..n {
        for j in i + 1..=n {
            if is_palindrome(&s[i..j]) {
                res = res.max(j - i);
            }
        }
    }

    println!("{res}")
}
