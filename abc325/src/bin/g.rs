use proconio::*;

fn naive(mut s: Vec<char>, k: usize) -> usize {
    let mut now = s.len();
    while now > 0 {
        now -= 1;
        if now + 1 < s.len() && s[now] == 'o' && s[now + 1] == 'f' {
            s = [s[..now].to_vec(), s[(now + 1 + k).min(s.len())..].to_vec()].concat();
        }
    }

    s.len()
}

fn main() {
    input! {s: marker::Chars, k: usize}

    let _res = naive(s.clone(), k);
}
