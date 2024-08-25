use std::{collections::VecDeque, io::Write, thread::sleep, time::Duration};

use proconio::*;

fn main() {
    input_interactive!(n: usize, l: usize, r: usize);
    eprintln!("n: {n}, l: {l}, r: {r}");

    let max = 1 << n;
    let mut d = vec![usize::MAX; max + 1];
    d[l] = 0;
    let mut prev = vec![usize::MAX; max + 1];
    let mut nt = VecDeque::new();
    nt.push_back(l);
    while let Some(now) = nt.pop_front() {
        let tr = now.trailing_zeros() as usize;
        for i in 0..=tr.min(32) {
            let next = now + (1 << i);
            if next <= max && d[next] == usize::MAX {
                d[next] = d[now] + 1;
                prev[next] = now;
                nt.push_back(next);
            }
            if now >= 1 << i {
                let next = now - (1 << i);
                if d[next] == usize::MAX {
                    d[next] = d[now] + 1;
                    prev[next] = now;
                    nt.push_back(next);
                }
            }
        }
    }

    let mut res = 0;
    let mut now = r + 1;
    while now != l {
        let p = prev[now];
        let (l, r) = (p.min(now), p.max(now));
        let (mut i, mut j, mut k) = (0, l, r);
        while k - j > 1 {
            i += 1;
            j >>= 1;
            k >>= 1;
        }
        assert_eq!((1 << i) * j, l);
        assert_eq!((1 << i) * (j + 1), r);
        assert_eq!(k - j, 1);
        println!("? {i} {j}");
        std::io::stdout().flush().unwrap();
        input_interactive!(t: i32);
        if t < 0 {
            sleep(Duration::from_millis(10000));
        }
        if p < now {
            res += t;
        } else {
            res -= t;
        }
        now = p;
    }

    println!("! {}", res.rem_euclid(100));
    std::io::stdout().flush().unwrap();
}
