use proconio::*;
use std::collections::VecDeque;

fn main() {
    input! {s: marker::Chars, q: usize}

    let mut d = true;
    let mut res = s.into_iter().collect::<VecDeque<_>>();
    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            d = !d;
        } else {
            input! {f: usize, c: char}
            if d {
                if f == 1 {
                    res.push_front(c);
                } else {
                    res.push_back(c);
                }
            } else {
                if f == 1 {
                    res.push_back(c);
                } else {
                    res.push_front(c);
                }
            }
        }
    }

    let res = if d {
        res.iter().collect::<String>()
    } else {
        res.iter().rev().collect::<String>()
    };
    println!("{}", res)
}
