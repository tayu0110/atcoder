use std::collections::BinaryHeap;

#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, x: usize, y: usize, z: usize, a: [usize; n], b: [usize; n]};

    let mut nt = BinaryHeap::new();
    for i in 0..n {
        nt.push((a[i], n - i, b[i]));
    }

    let mut res = vec![];
    for _ in 0..x {
        let (_, ni, _) = nt.pop().unwrap();
        res.push(n - ni);
    }
    let mut nnt = BinaryHeap::new();
    while let Some((_, i, _)) = nt.pop() {
        nnt.push((b[n-i], i, a[n-i]));
    }
    for _ in 0..y {
        let (_, ni, _) = nnt.pop().unwrap();
        res.push(n - ni);
    }

    let mut nnnt = BinaryHeap::new();
    while let Some((_, ni, _)) = nnt.pop() {
        nnnt.push((a[n - ni]+b[n - ni], ni));
    }
    for _ in 0..z {
        let (_, ni) = nnnt.pop().unwrap();
        res.push(n - ni);
    }

    res.sort();
    for v in res {
        println!("{}", v+1);
    }
}
