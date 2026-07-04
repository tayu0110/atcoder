use proconio::*;

fn main() {
    input! {s: marker::Bytes}
    let n = s.len();

    let mut ret = 0;
    for i in 0..n {
        let b = s[i];
        if b == b'C' {
            let min = i.min(n - 1 - i);
            ret += min + 1;
        }
    }
    println!("{ret}");
}
