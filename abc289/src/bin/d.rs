#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, a: [usize; n], m: usize, b: [usize; m], x: usize}

    let set = b.into_iter().collect::<std::collections::HashSet<_>>();

    let mut dp = vec![false; x + 1];
    dp[0] = true;

    for i in 0..x {
        if !dp[i] {
            continue;
        }
        if set.contains(&i) {
            continue;
        }

        for &a in &a {
            if i + a <= x {
                dp[i + a] = true;
            }
        }
    }

    if dp[x] {
        println!("Yes")
    } else {
        println!("No")
    }
}
