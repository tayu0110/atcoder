use proconio::input;

fn main() {
    input! {n: usize, x: usize, y: usize}
    println!("{}", (1..=n).filter(|v| v % x == 0 || v % y == 0).count())
}
