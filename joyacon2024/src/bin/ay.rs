use proconio::*;

fn main() {
    input! {mut n: usize}

    let s = n % 60;
    n /= 60;
    let m = n % 60;
    n /= 60;

    println!("{n:02}:{m:02}:{s:02}")
}
