use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {l: usize, r: usize, m: usize};

    let m = {
        let mut res = 1usize;
        for _ in 0..m {
            res *= 10;
        }
        res
    };

    let mut ck = BTreeSet::new();
    let mut buf = vec![];
    let mut now = 1usize;
    loop {
        buf.push(now);
        if ck.contains(&now) {
            break;
        }
        ck.insert(now);
        now *= 5;
        now %= m;
    }

    let g = {
        let mut res = 0;
        let back = *buf.last().unwrap();
        for (i, v) in buf.iter().enumerate() {
            if *v == back {
                res = i;
                break;
            }
        }
        res
    };

    eprintln!("{:?}", buf);
    eprintln!("g: {}", g);

    let (l, r) = {
        let l = if l >= g { l - g } else { 1 };
        let r = if r >= g { r - g } else { 1 };
        (l, r)
    };
    let buf = &buf[g..buf.len()-1];
    let len = buf.len();

    let mut res = 0;
    let (mut nl, mut nr) = ((l-1) / len, r / len);
    for v in buf {
        let t = nr - nl;
        let sum = {
            let mut res = 0;
            let mut v = *v;
            while v > 0 {
                res += v % 10;
                v /= 10;
            }
            res
        };
        res += t * sum;
        nl += 1;
        nr += 1;
        if nr > r {
            nr -= len;
        }
    }

    println!("{}", res);
}