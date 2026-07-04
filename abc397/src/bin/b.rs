use proconio::*;

fn main() {
    input! {mut s: marker::Bytes}

    let mut res = 0;
    let mut i = 0;
    while i < s.len() {
        if i % 2 == 0 {
            if s[i] != b'i' {
                s.insert(i, b'i');
                res += 1;
            }
        } else {
            if s[i] != b'o' {
                s.insert(i, b'o');
                res += 1;
            }
        }
        i += 1;
    }

    if s.len() % 2 != 0 {
        res += 1;
    }

    println!("{res}")
}
