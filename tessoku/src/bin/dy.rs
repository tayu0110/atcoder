use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, x: usize, mut a: marker::Bytes}

    let mut nt = VecDeque::new();
    nt.push_back(x - 1);
    a[x - 1] = b'@';
    while let Some(now) = nt.pop_front() {
        if now > 0 && a[now - 1] == b'.' {
            a[now - 1] = b'@';
            nt.push_back(now - 1);
        }
        if now + 1 < n && a[now + 1] == b'.' {
            a[now + 1] = b'@';
            nt.push_back(now + 1);
        }
    }
    println!("{}", a.into_iter().map(|b| b as char).collect::<String>())
}
