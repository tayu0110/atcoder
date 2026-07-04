use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut ret = 0usize;
    let mut prev = 0;
    for i in 1..s.len() {
        if s[i] == s[i - 1] {
            let diff = i - prev;
            prev = i;
            ret += diff * (diff + 1) / 2;
            ret %= 998244353;
        }
    }
    let diff = s.len() - prev;
    ret += diff * (diff + 1) / 2;
    ret %= 998244353;

    println!("{}", ret);
}
