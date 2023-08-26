use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {_: usize, m: usize, mut h: i32, k: i32, s: marker::Chars, p: [(i32, i32); m]}

    let mut set = p.into_iter().collect::<HashSet<_>>();
    let mut now = (0, 0);
    for c in s {
        match c {
            'U' => now.1 += 1,
            'D' => now.1 -= 1,
            'R' => now.0 += 1,
            'L' => now.0 -= 1,
            _ => unreachable!(),
        }

        h -= 1;

        if h < 0 {
            println!("No");
            return;
        }

        if set.contains(&now) && h < k {
            set.remove(&now);
            h = h.max(k);
        }
    }

    println!("Yes")
}
