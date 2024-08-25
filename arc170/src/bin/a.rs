use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {_: usize, mut s: marker::Chars, mut t: marker::Chars}

    while let Some(c) = t.pop() {
        if c != 'A' {
            t.push(c);
            break;
        }
        let sc = s.pop().unwrap();
        if c != sc {
            println!("-1");
            return;
        }
    }
    s.reverse();
    t.reverse();
    while let Some(c) = t.pop() {
        if c != 'B' {
            t.push(c);
            break;
        }
        let sc = s.pop().unwrap();
        if c != sc {
            println!("-1");
            return;
        }
    }
    s.reverse();
    t.reverse();
    let n = s.len();
    if n == 0 {
        println!("0");
        return;
    }

    let mut res = 0;

    {
        let mut a = VecDeque::new();
        let mut b = VecDeque::new();
        for i in 0..n {
            if s[i] == 'B' && t[i] == 'A' {
                a.push_back(i);
            } else if s[i] == 'A' && t[i] == 'B' {
                b.push_back(i);
            }
        }

        while let Some(a) = a.pop_front() {
            while let Some(b) = b.pop_front() {
                if b < a {
                    continue;
                }

                s[a] = 'A';
                s[b] = 'B';
                res += 1;
                break;
            }
        }
    }

    res += s.into_iter().zip(t).filter(|(s, t)| s != t).count();
    println!("{res}")
}
