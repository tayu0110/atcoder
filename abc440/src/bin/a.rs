use proconio::*;

fn main() {
    input! {mut x: usize, y: usize}
    for _ in 0..y {
        x *= 2;
    }
    println!("{x}")
}
