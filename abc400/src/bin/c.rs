use math::MathInt;
use proconio::*;

fn main() {
    input! {n: usize}

    let mut res = 0;
    for i in 1..62 {
        let b2 = n / (1 << i);
        let b = b2.sqrti();
        res += (b + 1) / 2;
    }
    println!("{res}")
}
