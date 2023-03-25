use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let len = s.len();
    for len in (0..len).step_by(2).rev() {
        if s[..len / 2] == s[len / 2..len] {
            println!("{}", len);
            return;
        }
    }
}
