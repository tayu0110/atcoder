use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m], q: usize, xk: [(usize, usize); q]};

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
    }

    for (x, k) in xk {
        let x = x-1;
        let mut nt = VecDeque::new();
        let mut list = vec![];
        nt.push_back((x, 0));

        while !nt.is_empty() {
            let (now, d) = nt.pop_front().unwrap();
            if d > k {
                continue;
            }
            if list.contains(&(now+1)) {
                continue;
            }
            list.push(now+1);
            for to in &t[now] {
                nt.push_back((*to, d+1));
            }
        }
        println!("{}", list.iter().fold(0, |sum, x| sum + *x));
    }
}