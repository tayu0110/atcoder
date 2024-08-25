use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: String}

    let mut cnt = [0; 128];
    for c in s.bytes() {
        cnt[c as usize] += 1;
    }

    println!("{}", cnt[b'A' as usize..=b'F' as usize].iter().join(" "))
}
