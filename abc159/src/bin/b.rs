use proconio::*;

fn is_palindrome(s: &[char]) -> bool {
    let len = s.len();
    for i in 0..len {
        let j = len - 1 - i;
        if i > j {
            break;
        }
        if s[i] != s[j] {
            return false;
        }
    }
    true
}

fn main() {
    input! {s: marker::Chars}
    let n = s.len();

    if is_palindrome(&s) && is_palindrome(&s[..(n - 1) / 2]) && is_palindrome(&s[(n + 3) / 2 - 1..])
    {
        println!("Yes")
    } else {
        println!("No")
    }
}
