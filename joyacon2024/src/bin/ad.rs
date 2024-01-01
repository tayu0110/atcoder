use proconio::*;

fn main() {
    input! {r: i64, d: i64, mut x: i64}

    for _ in 0..10 {
        x = x * r - d;
        println!("{x}");
    }
}
