#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn f(h: usize, x: Vec<usize>) -> Vec<usize> {
    let mut zero = 0;
    for (i, x) in x.iter().cloned().map(|x| x - 1).enumerate() {
        if i % 2 == 0 {
            let new = (zero + x) % h;
            zero = new;
        } else {
            let new = (zero + h - x) % h;
            zero = new;
        }
    }

    if x.len() % 2 == 0 {
        (zero..h).into_iter().chain((0..zero).into_iter()).collect()
    } else {
        if zero == h - 1 {
            (0..h).into_iter().rev().collect()
        } else {
            (0..=zero)
                .into_iter()
                .rev()
                .chain((zero + 1..h).into_iter().rev())
                .collect()
        }
    }
}

fn main() {
    input! {h: usize, w: usize, a: [Chars; h], q: usize, p: [(usize, usize); q]}

    let (x, y) = p
        .into_iter()
        .unzip::<usize, usize, Vec<usize>, Vec<usize>>();
    let ver = f(h, x);
    let hor = f(w, y);

    for i in 0..h {
        let mut res = vec![];
        for j in 0..w {
            res.push(a[ver[i]][hor[j]]);
        }
        println!("{}", res.iter().join(""))
    }
}
