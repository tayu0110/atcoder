use proconio::*;

fn main() {
    input! {c: usize, h: usize}
    if c.max(h) >= 2800 {
        println!("o")
    } else {
        println!("x")
    }
}
