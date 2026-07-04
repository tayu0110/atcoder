use std::collections::BinaryHeap;

use cpio::*;

fn main() {
    scan! {t: usize}

    for _ in 0..t {
        scan! {n: usize, a: [u32; n * 2]}

        let mut res = a[0] as usize;
        let mut nt = BinaryHeap::new();
        for v in a[1..].chunks_exact(2) {
            nt.push(v[0]);
            nt.push(v[1]);
            res += nt.pop().unwrap() as usize;
        }

        putln!(res);
    }
}
