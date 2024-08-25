use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: u32, q: usize}

    let get_id = |s: &[u8]| -> usize { s.iter().fold(0usize, |s, v| (s * 3) + *v as usize) };
    let mut memo = vec![u32::MAX; 3usize.pow(n)];
    for _ in 0..q {
        input! {mut s: marker::Bytes}
        s.iter_mut().for_each(|s| *s -= b'A');

        let id = get_id(&s);
        if memo[id] < u32::MAX {
            println!("{}", memo[id]);
            continue;
        }

        let mut nt = VecDeque::new();
        let mut t = s.clone();
        t.sort_unstable();
        nt.push_back((0, t.clone()));
        memo[get_id(&t)] = 0;
        while let Some((now, s)) = nt.pop_front() {
            for i in 2..=s.len() {
                let mut next = s.clone();
                next[..i].reverse();
                let nid = get_id(&next);
                if memo[nid] == u32::MAX {
                    memo[nid] = now + 1;
                    nt.push_back((now + 1, next));
                }
            }
        }

        println!("{}", memo[id]);
    }
}
