use math::chinese_remainder_theorem;
use proconio::*;

fn main() {
    input! {x: i64, y: i64, z: i64}

    let (w, m) = chinese_remainder_theorem(y, 107, z, 1000000007).unwrap();
    println!("{}", chinese_remainder_theorem(x, 17, w, m).unwrap().0)
}
