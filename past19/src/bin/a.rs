use proconio::*;

fn main() {
    input! {x: usize, _: usize, m: usize}
    if m <= x {
        println!("{}", x - m)
    } else {
        println!("{}", 60 - (m - x))
    }
}
