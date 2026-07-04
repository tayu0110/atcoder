use std::time::Instant;

use itertools::Itertools;
use proconio::*;
use rand::{rng, seq::IndexedRandom};

fn main() {
    input! {n: usize, a: usize, b: usize, mut s: [marker::Bytes; n]}

    let mut buf = vec![];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] != b'#' {
                buf.push((i, j));
            }
        }
    }

    let mut rng = rng();
    let (mut l, mut r) = (0, buf.len() + 1);
    let mut ret = vec![];
    while r - l > 1 {
        let k = (r + l) / 2;
        let mut ok = false;
        let tm = Instant::now();
        while tm.elapsed().as_millis() < 400 {
            let choice = buf.choose_multiple(&mut rng, k).collect::<Vec<_>>();
            let mut bad = false;
            for i in 0..k {
                for j in i + 1..k {
                    let da = choice[i].0.abs_diff(choice[j].0);
                    let db = choice[i].1.abs_diff(choice[j].1);
                    if (da, db) == (a, b) || (da, db) == (b, a) {
                        bad = true;
                        break;
                    }
                }
            }

            if !bad {
                if ret.len() < choice.len() {
                    ret = choice.iter().copied().copied().collect::<Vec<_>>();
                }
                ok = true;
                break;
            }
        }

        if ok {
            l = k;
        } else {
            r = k;
        }
    }

    for (r, c) in ret {
        s[r][c] = b'o';
    }

    for i in 0..n {
        println!("{}", s[i].iter().map(|&c| c as char).join(""));
    }
}
