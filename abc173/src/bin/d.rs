use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();

    let mut ring = VecDeque::new();
    ring.push_back(a.pop().unwrap());
    ring.push_front(a.pop().unwrap());

    let mut res = 0;
    res += ring[1];

    while let Some(a) = a.pop() {
        let (b, f) = (ring.pop_back().unwrap(), ring.pop_back().unwrap());
        res += f.min(b);
        ring.push_front(b);
        ring.push_front(a);
        ring.push_back(f);
    }

    println!("{}", res)
}
