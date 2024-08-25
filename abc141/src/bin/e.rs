#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, s: Chars};

    let (mut l, mut r) = (0, n + 1);
    while r - l > 1 {
        let len = (r + l) / 2;
        let mut map = std::collections::HashMap::new();
        for i in 0..n {
            if i + len > n {
                break;
            }
            let buf = s[i..i+len].iter().copied().collect::<String>();
            map.entry(buf).or_insert(vec![]).push(i);
        }

        let mut ok = false;
        for v in map.values() {
            if v.len() < 2 {
                continue;
            }
            let (f, s) = (v[0], *v.last().unwrap());
            ok |= s - f >= len;
        }

        if ok {
            l = len;
        } else {
            r = len;
        }
    }

    println!("{}", l);
}
