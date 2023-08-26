use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {t: usize}

    let mut r = vec![];
    for _ in 0..t {
        input! {n: usize, mut p: [(usize, usize, usize); n]}

        let mut res = 0;
        let mut red = vec![];
        let mut blue = vec![];
        for (k, l, r) in p.iter_mut() {
            *k -= 1;
            let min = *l.min(r);
            res += min;
            *l -= min;
            *r -= min;
            if l > r {
                red.push((*k, *l));
            } else if l < r {
                blue.push((*k + 1, *r));
            }
        }
        let mut nt = BinaryHeap::new();
        red.sort();
        let mut cursor = 0;
        for i in 0..n {
            while cursor < red.len() && red[cursor].0 == i {
                nt.push(Reverse(red[cursor].1));
                cursor += 1;
            }
            while nt.len() > i + 1 {
                nt.pop().unwrap();
            }
        }
        res += nt.into_iter().map(|Reverse(v)| v).sum::<usize>();

        let mut nt = BinaryHeap::new();
        blue.sort_by_key(|&(v, _)| Reverse(v));
        let mut cursor = 0;
        for i in (0..=n).rev() {
            while cursor < blue.len() && blue[cursor].0 == i {
                nt.push(Reverse(blue[cursor].1));
                cursor += 1;
            }
            while nt.len() > n - i {
                nt.pop().unwrap();
            }
        }
        res += nt.into_iter().map(|Reverse(v)| v).sum::<usize>();

        r.push(res);
    }

    for r in r {
        println!("{}", r)
    }
}
