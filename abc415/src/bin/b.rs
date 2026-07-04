use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut i = 0;
    while i < s.len() {
        while i < s.len() && s[i] != b'#' {
            i += 1;
        }
        if i == s.len() {
            break;
        }
        let j = i;
        i += 1;

        while i < s.len() && s[i] != b'#' {
            i += 1;
        }
        println!("{},{}", j + 1, i + 1);
        i += 1;
    }
}
