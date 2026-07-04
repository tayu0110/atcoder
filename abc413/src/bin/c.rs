use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {q: usize}

    let mut nt = VecDeque::new();
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {c: usize, x: usize}
            nt.push_back((c, x));
        } else {
            input! {k: usize}
            let mut sum = 0;
            let mut res = 0;
            while sum < k {
                let (c, x) = nt.pop_front().unwrap();
                if sum + c <= k {
                    sum += c;
                    res += c * x;
                } else {
                    res += x * (k - sum);
                    nt.push_front((c - (k - sum), x));
                    sum = k;
                }
            }
            println!("{res}")
        }
    }
}
