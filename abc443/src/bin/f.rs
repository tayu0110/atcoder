use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize}

    if n % 10 == 0 {
        println!("-1");
        return;
    }

    if n < 10 {
        println!("{}", n);
        return;
    }

    let mut ret = vec![vec![None::<(u32, u8)>; n]; 10];
    let mut next = VecDeque::new();
    for i in 1..10 {
        next.push_back((i, i));
        ret[i][i] = Some((u32::MAX, u8::MAX));
    }
    while let Some((back, rem)) = next.pop_front() {
        for i in back..10 {
            let t = rem * 10 + i;
            if t % n == 0 {
                ret[i][t % n] = Some((rem as u32, back as u8));
                let mut buf = String::new();
                fn printret(i: usize, r: usize, ret: &[Vec<Option<(u32, u8)>>], buf: &mut String) {
                    let (rem, back) = ret[i][r].unwrap();
                    if rem != u32::MAX {
                        printret(back as usize, rem as usize, ret, buf);
                    }
                    buf.push((i as u8 + b'0') as char);
                }
                printret(i, t % n, &ret, &mut buf);
                println!("{}", buf);
                return;
            }
            if ret[i][t % n].is_none() {
                next.push_back((i, t % n));
                ret[i][t % n] = Some((rem as u32, back as u8));
            }
        }
    }

    println!("-1");
}
