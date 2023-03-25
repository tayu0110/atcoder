use proconio::*;

fn main() {
    input! {x: usize, a: usize, b: usize}

    if a >= b {
        println!("delicious")
    } else if b - a <= x {
        println!("safe")
    } else {
        println!("dangerous")
    }
}
