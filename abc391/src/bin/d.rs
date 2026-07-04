use std::io::{stdout, Write};

use proconio::*;

fn main() {
    input! {n: usize, w: usize, p: [(i32, i32); n], q: usize, query: [(i32, i32); q]}

    let mut blocks = vec![vec![]; w + 1];
    for (i, &(x, y)) in p.iter().enumerate() {
        blocks[x as usize].push((y, i));
    }

    let mut cnt = vec![0; n];
    let mut disappear = vec![-1; n + 1];
    for x in 1..=w {
        blocks[x].sort_unstable();

        for (j, &(y, i)) in blocks[x].iter().enumerate() {
            cnt[i] = j as i32;
            disappear[j] = disappear[j].max(y as i32);
        }
        disappear[blocks[x].len()] = i32::MAX;
    }
    for i in 0..n {
        disappear[i + 1] = disappear[i + 1].max(disappear[i].saturating_add(1));
    }

    let mut out = stdout().lock();
    for (t, a) in query {
        if (t as i32) < disappear[cnt[a as usize - 1] as usize] {
            writeln!(out, "Yes").unwrap();
        } else {
            writeln!(out, "No").unwrap();
        }
    }
}
