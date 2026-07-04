use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let n = s.len();
    let mut res = 0;
    for width in 1..=n {
        for i in 0..n {
            let j = i + width;
            let k = j + width;
            if k < n && s[i] == b'A' && s[j] == b'B' && s[k] == b'C' {
                res += 1;
            }
        }
    }

    println!("{res}")
}
