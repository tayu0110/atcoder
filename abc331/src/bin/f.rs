use proconio::*;
use string::DynamicRollingHash;

fn main() {
    input! {n: usize, q: usize, s: String}

    let mut hash = DynamicRollingHash::new(&s);
    let mut rev = DynamicRollingHash::new(&s.chars().rev().collect::<String>());

    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {x: usize, c: char}
            hash.set(x - 1, c);
            rev.set(n - x, c);
        } else {
            input! {l: usize, r: usize}
            let l = l - 1;

            if hash.get(l..r) == rev.get(n - r..n - l) {
                println!("Yes")
            } else {
                println!("No")
            }
        }
    }
}
