#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, a: [usize; n], q: i32}

    // (val, cleared)
    let mut a = a.into_iter().map(|c| (c, -1i32)).collect::<Vec<(_, _)>>();

    let mut res = vec![];
    let mut cleared = -1;
    let mut base = 0;
    for turn in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: usize}

            cleared = turn;
            base = x;
        } else if t == 2 {
            input! {j: usize, x: usize}
            let j = j - 1;

            if a[j].1 < cleared {
                a[j] = (base + x, turn);
            } else {
                a[j].0 += x;
            }
        } else {
            input! {j: usize}
            let j = j - 1;

            if a[j].1 < cleared {
                res.push(base);
            } else {
                res.push(a[j].0);
            }
        }
    }

    for res in res {
        println!("{}", res);
    }
}
