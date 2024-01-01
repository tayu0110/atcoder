use num::integer::Roots;
use proconio::*;

fn main() {
    input! {n: u64}

    let (mut l, mut r) = (0, n.sqrt() * 2);
    while r - l > 1 {
        let m = (r + l) / 2;
        if m * (m + 1) / 2 <= n {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{l}")
}
