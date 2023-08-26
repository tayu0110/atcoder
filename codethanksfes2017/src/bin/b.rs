use proconio::*;

fn solve(len: usize, mut s: Vec<char>) -> Vec<char> {
    s.resize(len, 'a');
    for i in 0..len {
        let j = len - 1 - i;
        if i > j {
            break;
        }

        if s[i] == s[j] {
            continue;
        }
        s[j] = s[i];
    }

    s
}

fn main() {
    input! {s: marker::Chars}
    let len = s.len();
    for i in 0..=len {
        let t = solve(len + i, s.clone());
        if s[..] == t[..len] {
            println!("{}", i);
            return;
        }
    }
}
