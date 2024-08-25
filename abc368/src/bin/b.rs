use std::cmp::Reverse;

use proconio::*;

fn main() {
    input! {n: usize, mut a: [i32; n]}

    for i in 1.. {
        a.sort_unstable_by_key(|&a| Reverse(a));
        a[0] -= 1;
        a[1] -= 1;

        if a.iter().filter(|&&a| a > 0).count() <= 1 {
            println!("{i}");
            return;
        }
    }
}
