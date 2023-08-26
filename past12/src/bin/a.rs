use proconio::*;

fn main() {
    input! {x: usize, y: usize, z: usize}
    println!("{}", y.max(x + z))
}
