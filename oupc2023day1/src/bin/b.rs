use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let n = s.len();
    let mut res = 0;
    let mut buf = vec![];
    for c in s {
        match buf.last() {
            Some(&d) if d == c => {
                res += buf.len() * (buf.len() + 1) / 2;
                buf.clear();
                buf.push(c);
            }
            _ => buf.push(c),
        }
    }

    res += buf.len() * (buf.len() + 1) / 2;
    println!("{}", (n * (n + 1) / 2 - res) % 998244353)
}
