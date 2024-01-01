use proconio::*;
use string::RollingHash;

fn main() {
    input! {n: usize, q: usize, s: String}

    let hash = RollingHash::new(s.as_str());

    let rev = s.chars().rev().collect::<String>();
    let rev = RollingHash::new(rev.as_str());

    for _ in 0..q {
        input! {mut l: usize, r: usize}
        l -= 1;

        println!(
            "{}",
            (hash.get(l..r) == rev.get(n - r..n - l))
                .then_some("Yes")
                .unwrap_or("No")
        )
    }
}
