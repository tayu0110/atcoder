use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {k: usize}
    let mut set = HashSet::new();
    let mut now = 7 % k;
    for i in 1.. {
        if now == 0 {
            println!("{}", i);
            return;
        }
        set.insert(now);

        now = (now * 10 + 7) % k;
        if set.contains(&now) {
            println!("-1");
            return;
        }
    }
}
