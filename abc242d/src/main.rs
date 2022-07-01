use proconio::input;
use proconio::marker::*;

fn main() {
    input! {s: Chars, q: usize, p: [(usize, usize); q]};

    for (mut t, mut k) in p {
        let mut inst = vec![];
        k -= 1;
        while t > 0 && k > 0 {
            inst.push(k % 2);
            k /= 2;
            t -= 1;
        }
        inst.reverse();
        let mut now = s[k];
        if t % 3 == 1 {
            if now == 'A' {
                now = 'B';
            } else if now == 'B' {
                now = 'C';
            } else {
                now = 'A';
            }
        } else if t % 3 == 2 {
            if now == 'A' {
                now = 'C';
            } else if now == 'B' {
                now = 'A';
            } else {
                now = 'B';
            }
        }
        for v in inst {
            if v == 0 {
                if now == 'A' {
                    now = 'B';
                } else if now == 'B' {
                    now = 'C';
                } else {
                    now = 'A';
                }
            } else {
                if now == 'A' {
                    now = 'C';
                } else if now == 'B' {
                    now = 'A';
                } else {
                    now = 'B';
                }
            }
        }
        println!("{}", now);
    }
}