use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;
use unionfind::UnionFind;

#[fastout]
fn main() {
    input! {n: usize, q: usize, mut e: [(usize, usize); n]}

    let mut nt = BinaryHeap::new();
    for (i, &(u, v)) in e.iter().enumerate() {
        for (j, &(nu, nv)) in e.iter().take(i).enumerate() {
            nt.push(Reverse((u.abs_diff(nu) + v.abs_diff(nv), i, j)));
        }
    }
    let mut m = n;
    let mut uf = UnionFind::new(n + q);
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {a: usize, b: usize}
            for (i, &(u, v)) in e.iter().enumerate() {
                nt.push(Reverse((a.abs_diff(u) + b.abs_diff(v), i, e.len())));
            }
            e.push((a, b));
            m += 1;
        } else if ty == 2 {
            if m == 1 {
                println!("-1");
                continue;
            }

            while nt
                .peek()
                .is_some_and(|&Reverse((_, i, j))| uf.is_same(i, j))
            {
                nt.pop();
            }
            if let Some(Reverse((d, i, j))) = nt.pop() {
                m -= 1;
                uf.merge(i, j);
                while let Some(&Reverse((_, i, j))) =
                    nt.peek().filter(|&Reverse((nd, _, _))| *nd == d)
                {
                    nt.pop();
                    if uf.merge(i, j) {
                        m -= 1;
                    }
                }
                println!("{d}");
            }
        } else {
            input! {u: usize, v: usize}
            if uf.is_same(u - 1, v - 1) {
                println!("Yes")
            } else {
                println!("No")
            }
        }
    }
}
