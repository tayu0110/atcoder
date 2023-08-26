use proconio::*;

fn main() {
    input! {mut n: usize}

    let mut t = vec![];
    while n > 0 {
        if n & 1 != 0 {
            t.push('A');
        }
        t.push('B');
        n >>= 1;
    }

    println!("{}", t.into_iter().rev().collect::<String>())
}
