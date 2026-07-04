use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let (mut l, mut r) = (0, n + 1);
    'o: while r - l > 1 {
        let m = (r + l) / 2;
        if m >= n {
            r = m;
            continue;
        }
        let mut nt = a[m..].iter().cloned().collect::<VecDeque<_>>();
        'b: for i in 0..m {
            while let Some(now) = nt.pop_front() {
                if a[i] * 2 <= now {
                    continue 'b;
                }
            }
            r = m;
            continue 'o;
        }
        l = m;
    }
    println!("{l}")
}
