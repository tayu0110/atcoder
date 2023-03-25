#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    let mut b = (0..=n).collect::<Vec<_>>();
    let mut look = vec![0; m];
    let mut now_one = 1;

    for (i, a) in a.into_iter().enumerate() {
        b.swap(a, a + 1);
        look[i] = b[now_one];
        if b[now_one] != 1 {
            if b[a] == 1 {
                now_one = a;
            } else {
                now_one = a + 1;
            }
        }
    }

    let mut pos = vec![0; n + 1];
    for i in 1..n + 1 {
        pos[b[i]] = i;
    }

    for i in 0..m {
        println!("{}", pos[look[i]]);
    }
}
