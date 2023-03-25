#![allow(unused_imports)]
use itertools::Itertools;
use math::chinese_remainder_theorem;
use proconio::{input, source::line::LineSource};
use std::io::{self, BufReader};

const M: [usize; 9] = [4, 5, 7, 9, 11, 13, 17, 19, 23];

fn main() {
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    let m = M.iter().sum::<usize>();
    println!("{}", m);
    let a = {
        let mut buf = vec![];
        for &m in M.iter() {
            let base = buf.len();
            for i in 0..m {
                buf.push((i + m - 1) % m + base);
            }
        }
        for a in buf.iter_mut() {
            *a += 1;
        }
        buf
    };
    assert_eq!(a.len(), m);
    println!("{}", a.iter().join(" "));

    input! {b: [usize; m]}

    let mut head = 0;
    let mut cond = vec![];
    for &k in M.iter() {
        let tail = head + k;
        let t = &b[head..tail];
        for (i, &c) in t.iter().enumerate() {
            // eprintln!("i: {}, c: {}, head: {}", i, c, head);
            if c - head == 1 {
                cond.push((i as i64, k as i64));
                break;
            }
        }
        head = tail;
    }

    let (mut res, mut lcm) = (0, 1);
    for (a, modulo) in cond {
        if lcm == 1 {
            res = a;
            lcm = modulo;
            continue;
        }

        let (r, l) = chinese_remainder_theorem(res, lcm, a, modulo).unwrap();
        res = r;
        lcm = l;
    }

    println!("{}", res);
}
